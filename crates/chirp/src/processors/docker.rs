use std::collections::HashMap;

use anyhow::bail;
use bollard::{
    container::{Config, RemoveContainerOptions},
    exec::{CreateExecOptions, StartExecResults},
    image::CreateImageOptions,
    Docker, API_DEFAULT_VERSION,
};
use futures::{StreamExt, TryStreamExt};
use tracing::debug;

pub async fn run_docker(image: &str, commands: &HashMap<String, String>) -> anyhow::Result<()> {
    let docker = Docker::connect_with_unix(
        "/Users/geoffreymureithi/.colima/default/docker.sock",
        120,
        API_DEFAULT_VERSION,
    )
    .unwrap();

    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: image,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await?;

    let alpine_config = Config {
        image: Some(image),
        tty: Some(true),
        ..Default::default()
    };

    let id = docker
        .create_container::<&str, &str>(None, alpine_config)
        .await?
        .id;
    docker.start_container::<String>(&id, None).await?;

    for command in commands.values() {
        // non interactive
        let exec = docker
            .create_exec(
                &id,
                CreateExecOptions {
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    cmd: Some(shlex::split(command).unwrap()),
                    ..Default::default()
                },
            )
            .await?
            .id;
        if let StartExecResults::Attached { mut output, .. } =
            docker.start_exec(&exec, None).await?
        {
            while let Some(msg) = output.next().await {
                debug!("{msg:?}");
            }
        } else {
            unreachable!();
        }
        let info = docker.inspect_exec(&exec).await?;
        if info.exit_code.unwrap() != 0 {
            docker
                .remove_container(
                    &id,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await?;
            bail!("command failed with code: {}", info.exit_code.unwrap())
        }
    }

    docker
        .remove_container(
            &id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await?;

    Ok(())
}
