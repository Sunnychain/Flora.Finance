import React from 'react';
import { useSubstrate } from '../../substrate-lib';
function NewAuction (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  async function createAuction () {
    accountPair !== ''
      ? console.log(accountPair.meta.name)
      : console.log('not user');
    const auction = await api.query.nftMarket.auctionsInfo.toJSON();
    console.log(auction);
  }
  return (
        <button onClick={createAuction}>
            Create Auction
        </button>
  );
}

export default NewAuction;
