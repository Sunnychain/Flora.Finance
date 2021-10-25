import React, { useState, createRef } from 'react';
import { Container, Grid, Sticky, Dimmer, Loader, Message } from 'semantic-ui-react';
import { useSubstrate } from '../substrate-lib';
import { DeveloperConsole } from '../substrate-lib/components';
import AccountSelector from '../components/AccountSelector';
import Events from '../components/Events';
import Interactor from '../components/Interactor';
import 'semantic-ui-css/semantic.min.css';

import CreateAuction from '../components/createAuction/NewAuction';
import { message, loader } from './status';
import Home from '../pages/Home/Home';
export default function Main () {
  const [accountAddress, setAccountAddress] = useState(null);
  const { apiState, keyring, keyringState, apiError } = useSubstrate();
  const accountPair =
    accountAddress &&
    keyringState === 'READY' &&
    keyring.getPair(accountAddress);

  const loader = text =>
    <Dimmer active>
      <Loader size='small'>{text}</Loader>
    </Dimmer>;

  const message = err =>
    <Grid centered columns={2} padded>
      <Grid.Column>
        <Message negative compact floating
          header='Error Connecting to Substrate'
          content={`${JSON.stringify(err, null, 4)}`}
        />
      </Grid.Column>
    </Grid>;

  if (apiState === 'ERROR') return message(apiError);
  else if (apiState !== 'READY') return loader('Connecting to Substrate');

  if (keyringState !== 'READY') {
    return loader('Loading accounts (please review any extension\'s authorization)');
  }

  const contextRef = createRef();
  return (
    <div ref={contextRef}>
      <Sticky context={contextRef}>

      </Sticky>
      <Container>
        <Grid stackable columns='equal'>
          <Grid.Row>
            <Interactor accountPair={accountPair} />
          </Grid.Row>
        </Grid>
      </Container>
      <DeveloperConsole />
      <Home setAccountAddress={setAccountAddress} />
    </div>
  );
}
