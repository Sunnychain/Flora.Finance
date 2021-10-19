import React from 'react';
import { Web3ReactProvider } from '@web3-react/core';
import Web3 from 'web3';
import Wallet from './wallet';

export default function Home (Component, pageProps) {
  function getLibrary (provider) {
    return new Web3(provider);
  }

  return (
    <Web3ReactProvider getLibrary={getLibrary}>
      <Wallet {...pageProps} />
    </Web3ReactProvider>
  );
}
