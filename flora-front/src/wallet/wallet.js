import React, { useEffect } from 'react';
import { useWeb3React } from '@web3-react/core';
import { injected } from './connector';
import './wallet.scss';

export default function Wallet () {
  const { active, account, library, activate, deactivate } = useWeb3React();

  async function connect () {
    try {
      await activate(injected);
    } catch (err) {
      console.log(err);
    }
  }
  async function disconnect () {
    try {
      await deactivate();
    } catch (err) {
      console.log(err);
    }
  }

  useEffect(() => {
    connect();
  }, []);

  return (
    <main>
      <div>
        {
          active
            ? ''
            : <button className="button" onClick={ connect }>Connect wallet </button>
        }
        {
          account
            ? <span> <b>Connected with</b> { account } { console.log('conta', library)}</span>
            : ''
        }
        {
          account
            ? <button className="button" onClick={ disconnect } >Disconnect </button>
            : ''
        }
      </div>

    </main>
  );
}
