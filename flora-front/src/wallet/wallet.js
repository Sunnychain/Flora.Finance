import React, { useEffect, useState } from 'react';
import { useWeb3React } from '@web3-react/core';
import { injected } from './connector';
import { Button } from 'semantic-ui-react';
import Modal from 'react-bootstrap/Modal'

export default function Wallet () {
  const { active, account, activate, deactivate } = useWeb3React();
  const [show, setShow] = useState(true);
  const handleClose = () => setShow(false);
  const handleShow = () => setShow(true);

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
      <div style={ { marginTop: '25px', textAlign: 'center' } }>
        {
          active
            ? ''
            : <Button positive onClick={ connect }>Connect wallet </Button>
        }
        {
          account
            ? <span style={ { margin: '25px' } }> <b>Connected with</b> { account } </span>
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
