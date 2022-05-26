import React from 'react';
import { render } from 'react-dom';

import './index.css';
import './theme.css';
import './toastify.css';
import { BrowserRouter } from 'react-router-dom';
import { App } from './components/App';
import { Api } from './services/Api';

const basePath = ((window as any).__basePath__ =
  document.head.querySelector('base')?.getAttribute('href') || '');
const api = new Api({ basePath: 'http://127.0.0.1:8000' });

render(
  <BrowserRouter basename={basePath}>
    <App api={api} />
  </BrowserRouter>,
  document.getElementById('root')
);
