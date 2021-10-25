import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { useSubstrate } from '../../substrate-lib';
import Content from '../../components/Content/Content';
import Wallet from '../../wallet/main';
import AccountSelector from '../../components/AccountSelector';
import SideBar from '../../components/SideBarMenu/SideBar';
import close from '../../images/min_Flora-black.png';
import './Home.scss';
import { message, loader } from '../../middlewares/status';
export default function Home (props) {
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

  return (
    <main>
      <div className="accountHeader">
      <AccountSelector setAccountAddress={setAccountAddress} />
        <nav>
          <ul className="nav justify-content-end">

            <li className="nav-item">
              {
                window.location.pathname === '/market'
                  ? <Wallet />
                  : ''
              }
            </li>
          </ul>
        </nav>
      </div>
      <div className='page-wrapper chiller-theme toggled'>
        <Link id='show-sidebar' to='#'>
         <img src={close} alt="open"className="openMenu" width="100px" />
        </Link>
        <SideBar />
        <Content />
      </div>
    </main>

  );
}
