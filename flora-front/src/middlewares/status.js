import { Dimmer, Loader, Grid, Message } from 'semantic-ui-react';

export const message = err => {
  return (
    <Grid centered columns={2} padded>
      <Grid.Column>
        <Message negative compact floating
          header='Error Connecting to Substrate'
          content={`${JSON.stringify(err, null, 4)}`}
        />
      </Grid.Column>
    </Grid>
  );
};

export const loader = () => {
  return (
    <Dimmer active>
      <Loader size='small'>Flora.Finance</Loader>
    </Dimmer>
  );
};
