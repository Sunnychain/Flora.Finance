import React from 'react';
import { Link } from 'react-router-dom';
import Content from '../../components/Content/Content';
import Wallet from '../../wallet/main';
import SideBar from '../../components/SideBarMenu/SideBar';
import close from '../../images/min_Flora-black.png';
import './Home.scss';

export default function Home () {
  return (
    <main>
      <div className="accountHeader">
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
        {
          window.location.pathname === '/' || window.location.pathname === 'home' ? <Content /> : ''
        }

      </div>
    </main>

  );
}
