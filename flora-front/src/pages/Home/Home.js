import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { MdCollectionsBookmark } from 'react-icons/md';
import { RiHandHeartLine } from 'react-icons/ri';
import { GiBuyCard } from 'react-icons/gi';
import Content from '../../components/Content/Content';
import Wallet from '../../wallet/main';
import $ from 'jquery';
import './Home.scss';

export default function Home () {
  function renderSizer () {
    $('.sidebar-dropdown > a').click(function () {
      $('.sidebar-submenu').slideUp(200);
      if (
        $(this)
          .parent()
          .hasClass('active')
      ) {
        $('.sidebar-dropdown').removeClass('active');
        $(this)
          .parent()
          .removeClass('active');
      } else {
        $('.sidebar-dropdown').removeClass('active');
        $(this)
          .next('.sidebar-submenu')
          .slideDown(200);
        $(this)
          .parent()
          .addClass('active');
      }
    });
    $('#close-sidebar').click(function () {
      $('.page-wrapper').removeClass('toggled');
    });
    $('#show-sidebar').click(function () {
      $('.page-wrapper').addClass('toggled');
    });
  }
  useEffect(() => {
    renderSizer();
  }, []);
  return (
    <main>

      <div className="accountHeader">
        <nav>
          <ul className="nav justify-content-end">
            <li className="nav-item">
              <Link className="nav-link active" to="#">Active</Link>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="#">Link</Link>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="#">Link</Link>
            </li>
            <li className="nav-item">
              <Wallet />
            </li>
          </ul>
        </nav>
      </div>
      <div className='page-wrapper chiller-theme toggled'>
        <Link id='show-sidebar' className='btn btn-sm btn-dark' to='#'>
          <i className='fas fa-bars'></i>
        </Link>
        <nav id='sidebar' className='sidebar-wrapper'>
          <div className='sidebar-content'>
            <div className='sidebar-brand'>
              <Link to='#'>Flora.Finance</Link>
              <div id='close-sidebar'>
                <i className='fas fa-times'></i>
              </div>
            </div>
            <div className='sidebar-search'>

              <div className='input-group'>
                <input type='text' className='form-control search-menu' placeholder='Search...' />
                <div className='input-group-append'>
                  <span className='input-group-text'>
                    <i className='fa fa-search' aria-hidden='true'></i>
                  </span>
                </div>
              </div>

            </div>
            <div className='sidebar-menu'>
              <ul>
                <li className='header-menu'>
                  <span>General</span>
                </li>
                <li>
                  <Link to='#'>
                    <i className='fa fa-home'></i>
                    <span>Home</span>
                  </Link>
                </li>
                <li className='sidebar-dropdown'>
                  <Link to='#'>
                    <i className='fa fa-chart-line'></i>
                    <span>Trade</span>
                  </Link>
                  <div className='sidebar-submenu'>
                    <ul>
                      <li>
                        <Link to='#'>Liquidity</Link>
                      </li>
                      <li>
                        <Link to='#'>exchange</Link>
                      </li>
                    </ul>
                  </div>
                </li>
                <li>
                  <Link to='#'>
                    <i className='fa fa-tractor'></i>
                    <span>Farms</span>
                  </Link>
                </li>
                <li>
                  <Link to='#'>
                    <i className='fa fa-calendar'></i>
                    <span>Staking</span>
                  </Link>
                </li>

                <li>
                  <Link to='#'>
                    <i className='fa '><MdCollectionsBookmark /></i>
                    <span>NFTS</span>
                  </Link>
                </li>

                <li className='sidebar-dropdown'>
                  <Link to='#'>
                    <i className='fa '><GiBuyCard /></i>
                    <span>Games</span>
                  </Link>
                  <div className='sidebar-submenu'>
                    <ul>
                      <li>
                        <Link to='#'>Card Game</Link>
                      </li>
                    </ul>
                  </div>
                </li>
                <li>
                  <Link to='#'>
                    <i className='fa fa-hearth'><RiHandHeartLine /></i>
                    <span>Donations</span>
                  </Link>
                </li>

                <li className='header-menu'>
                  <span>Info</span>
                </li>
                <li>
                  <Link to='#'>
                    <i className='fa fa-book'></i>
                    <span>Project Info</span>
                    <span className='badge badge-pill badge-primary'>Beta</span>
                  </Link>
                </li>
                <li>
                  <Link to='#'>
                    <i className='fa fa-folder'></i>
                    <span>WhitePaper</span>
                  </Link>
                </li>
                <li>
                  <Link to='#'>
                    <i className='fa fa-folder'></i>
                    <span>contributors</span>
                  </Link>
                </li>
              </ul>
            </div>
          </div>

          {
            window.location.pathname === '/' ? '' : ''
          }

        </nav>
        <Content />
      </div>
    </main>

  );
}
