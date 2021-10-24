import React, { useState, createRef } from 'react';
import { Container, Grid, Sticky } from 'semantic-ui-react';
import { useSubstrate } from '../substrate-lib';
import { DeveloperConsole } from '../substrate-lib/components';
import AccountSelector from '../components/AccountSelector';
import Events from '../components/Events';
import Interactor from '../components/Interactor';
import { message, loader } from './status';
import 'semantic-ui-css/semantic.min.css';

export default function Main () {
  const [accountAddress, setAccountAddress] = useState(null);
  const { apiState, keyring, keyringState, apiError } = useSubstrate();
  const accountPair =
    accountAddress &&
    keyringState === 'READY' &&
    keyring.getPair(accountAddress);
  if (apiState === 'ERROR') return message(apiError);
  else if (apiState !== 'READY') return loader('Connecting to Substrate');
  if (keyringState !== 'READY') {
    return loader('Loading accounts (please review any extension\'s authorization)');
  }
  const contextRef = createRef();
  return (
      <div ref={contextRef}>
        <Sticky context={contextRef}>
        <AccountSelector setAccountAddress={setAccountAddress} />
        </Sticky>
        <Container>
          <Grid stackable columns='equal'>
            <Grid.Row>
            <Events />
              <Interactor accountPair={accountPair} />

            </Grid.Row>
          </Grid>
        </Container>
        <DeveloperConsole />
      </div>
  );
}
