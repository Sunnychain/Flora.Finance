import React from 'react';
import { Link } from 'react-router-dom';
import Wallet from '../../wallet/main';
import logo from '../../images/flora.svg';
import example from '../../images/example.jpeg';
import CreateAuction from '../../components/createAuction/NewAuction';

import './market.scss';

function Market () {
  return (
    <div id="mainWrapper">
      <header>
        <div id="logo"><img src={logo} alt="logo"/></div>
        <div id="headerLinks"><Wallet /></div>
      </header>
      <section id="offer">
        <div className="semi"></div>
        <h2>All our nfts represent real trees</h2>
        <h4>you can buy nfts according to your rarities</h4>
      </section>
      <div id="content">
        <section className="sidebar">

          <input type="text" id="search" placeholder="search" />
          <div id="menubar">
            <nav className="menu">
              Types
              <br />
              <h2>Rare</h2>
              <hr />
              <ul>
                <li><Link to='#' title="Link">Ruis&nbsp;&nbsp;</Link></li>
                <li><Link to='#' title="Link">Fearn</Link></li>
                <li><Link to='#' title="Link">Paw</Link></li>
              </ul>
            </nav>
            <nav className="menu">
              <h2>Ultra Rare</h2>
              <hr />
              <ul>

                <li><Link to='#' title="Link">Coll</Link></li>
                <li><Link to='#' title="Link">Ngetal</Link></li>
                <li><Link to='#' title="Link">Quert</Link></li>
                <li className="notimp"></li>
              </ul>
            </nav>
            <nav className="menu">
              <h2>Legendary </h2>
              <hr />
              <ul>
                <li><Link to='#' title="Link">Uath</Link></li>
                <li><Link to='#' title="Link">Nion</Link></li>
                <li><Link to='#' title="Link">Tinned</Link></li>
              </ul>
            </nav>
          </div>
        </section>
        <section className="mainContent">
          <div className="productRow">
            <article className="productInfo" style={{ width: 'auto' }}>
              <div><img alt="sample" src={example} /></div>
              <br/> <hr/>
              <p className="productContent">type </p>
              <p className="productContent">Metadata </p>
              <button type="button" name="button" defaultValue="Buy" className="buyButton"><b>$50</b></button>
            </article>

          </div>
        </section>
      </div>
      <br />
      <footer>
        <div>
          <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam varius sem neque. Integer ornare.</p>
        </div>
        <div>
          <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam varius sem neque. Integer ornare.</p>
        </div>
        <div className="footerlinks">
          <p><Link to='#' title="Link">Link to='#' 1 </Link></p>
          <p><Link to='#' title="Link">Link to='#' 2</Link></p>
          <p><Link to='#' title="Link">Link to='#' 3</Link></p>
        </div>
      </footer>
    </div>

  );
}

export default Market;
