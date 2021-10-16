import React from 'react';
import Web3 from 'web3';
import { Web3ReactProvider } from '@web3-react/core';
import Wallet from './wallet';

export default function Home () {
  function getLibrary (provider) {
    return new Web3(provider);
  }
  return (
    <Web3ReactProvider getLibrary={getLibrary}>
      <Wallet />
    </Web3ReactProvider>
  );
}
