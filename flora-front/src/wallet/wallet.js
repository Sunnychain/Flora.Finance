import React from 'react';
import { useWeb3React } from '@web3-react/core';
import { injected } from './connector';
import { Button } from 'semantic-ui-react';

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

  return (
    <main>
      <div style={ { marginTop: '5px', textAlign: 'center' } }>
        {
          active
            ? ''
            : <Button positive onClick={ connect }>Connect wallet </Button>
        }
        {
          account
            ? <span style={ { margin: '25px' } }> <b>Connected with</b> { account } { console.log('conta', library)}</span>
            : ''
        }
        {
          account
            ? <Button positive onClick={ disconnect } >Disconnect </Button>
            : ''
        }
      </div>

    </main>
  );
}
