import React from 'react';
import Main from './middlewares/Main';
import { SubstrateContextProvider } from './substrate-lib';
export default function App () {
  return (
    <SubstrateContextProvider>
        <Main />
    </SubstrateContextProvider>
  );
}
