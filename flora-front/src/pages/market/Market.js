import React, { useState } from 'react';
import Menu from '../Home/Home';
import { useSubstrate } from '../../substrate-lib';

function Market () {
  const { api } = useSubstrate();
  const [data, setData] = useState({});

  async function dale () {
    try {
      setData(await api.query.tokenNonFungible.collections(0));
      const dale = data.toJSON();
      console.log(dale);
    } catch (e) {
      console.log(e);
    }
  }

  return (
    <main>
      <Menu />
  <div>
    <h1 style={{ textAlign: 'center', marginTop: '-350px' }}>IN DEV</h1>
  </div>
  </main>
  );
}

export default Market;
