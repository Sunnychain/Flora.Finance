import React, { useState } from 'react';

import Interacotr from '../../components/Interactor';
import Phome from '../../components/Content/Phome';
import { MdCollectionsBookmark } from 'react-icons/md';
import { FcFeedback } from 'react-icons/fc';
import { DiGithubBadge } from 'react-icons/di';
import { RiHandHeartLine } from 'react-icons/ri';
import { GrProjects } from 'react-icons/gr';
import { FiBookOpen } from 'react-icons/fi';
import { GiBuyCard } from 'react-icons/gi';
import { Link } from 'react-router-dom';
import $ from 'jquery';
import './test.scss';

export default function test () {
  const [route, setRoute] = useState('');
  function initState () {
    setRoute('');
  }
  function handleChange (e) {
    setRoute($('#item1 span').text());
    console.log(route);
  }
  function renderContent () {
    if (route === '') { return <Phome />; }
    if (route === 'Collections') { return <Interacotr />; }
  }
  $(function () {
    $('.menu-link').click(function () {
      $('.menu-link').removeClass('is-active');
      $(this).addClass('is-active');
    });
  });

  $(function () {
    $('.main-header-link').click(function () {
      $('.main-header-link').removeClass('is-active');
      $(this).addClass('is-active');
    });
  });
  return (
    <main>
      <main className='main-app'>
        <div className='dark-light'>
          <svg viewBox='0 0 24 24' stroke='currentColor' stroke-width='1.5' fill='none' stroke-linecap='round' stroke-linejoin='round'>
            <path d='M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z' /></svg>
        </div>
        <div className='app'>
          <div className='header'>
            <div className='menu-circle'>Flora App</div>
            <div className='header-menu'>
              <Link className='menu-link is-active' to='#' onClick={initState}>Home</Link>
              <Link className='menu-link notify' to='#'>NFT'S</Link>
              <Link className='menu-link' to='#'>Trade</Link>
              <Link className='menu-link notify' to='#'>Auctions</Link>
            </div>
            <div className='search-bar'>
              <input type='text' placeholder='Search' />
            </div>
            <div className='header-profile'>
              <div className='notification'>
                <span className='notification-number'>3</span>
                <svg viewBox='0 0 24 24' fill='currentColor' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round' className='feather feather-bell'>
                  <path d='M18 8A6 6 0 006 8c0 7-3 9-3 9h18s-3-2-3-9M13.73 21a2 2 0 01-3.46 0' />
                </svg>
              </div>
              <svg viewBox='0 0 512 512' fill='currentColor'>
                <path d='M448.773 235.551A135.893 135.893 0 00451 211c0-74.443-60.557-135-135-135-47.52 0-91.567 25.313-115.766 65.537-32.666-10.59-66.182-6.049-93.794 12.979-27.612 19.013-44.092 49.116-45.425 82.031C24.716 253.788 0 290.497 0 331c0 7.031 1.703 13.887 3.006 20.537l.015.015C12.719 400.492 56.034 436 106 436h300c57.891 0 106-47.109 106-105 0-40.942-25.053-77.798-63.227-95.449z' />
              </svg>
              <img className='profile-img' src='#' alt='Avatar' />
            </div>
          </div>
          <div className='wrapper'>
            <div className='left-side'>
              <div className='side-wrapper'>
                <div className='side-title'>Flora</div>
                <div className='side-menu'>
                  <Link to='#' onClick={initState}>
                    <i className='fa fa-home'></i>
                    <span> Home </span>
                  </Link>
                  <Link to='#'>
                    <i><FiBookOpen /></i>
                    <span>About</span>
                  </Link>

                </div>
              </div>
              <div className='side-wrapper'>
                <div className='side-title'>Categories</div>
                <div className='side-menu'>
                  <Link to='#' onClick={handleChange}>
                    <i className='fa fa-chart-line'></i>
                    <span>Trade</span>
                  </Link>
                  <Link to='#'>
                    <svg viewBox='0 0 488.455 488.455' fill='currentColor'>
                      <path d='M287.396 216.317c23.845 23.845 23.845 62.505 0 86.35s-62.505 23.845-86.35 0-23.845-62.505 0-86.35 62.505-23.845 86.35 0' />
                      <path d='M427.397 91.581H385.21l-30.544-61.059H133.76l-30.515 61.089-42.127.075C27.533 91.746.193 119.115.164 152.715L0 396.86c0 33.675 27.384 61.074 61.059 61.074h366.338c33.675 0 61.059-27.384 61.059-61.059V152.639c-.001-33.674-27.385-61.058-61.059-61.058zM244.22 381.61c-67.335 0-122.118-54.783-122.118-122.118s54.783-122.118 122.118-122.118 122.118 54.783 122.118 122.118S311.555 381.61 244.22 381.61z' />
                    </svg>
                    NFT's
                  </Link>
                  <Link to='#'>
                    <i className='fa fa-tractor'></i>
                    <span>Farms</span>
                  </Link>
                  <Link to='#'>
                    <i className='fa fa-calendar'></i>
                    <span>Staking</span>
                  </Link>
                  <Link to="#" id="item1" onClick={handleChange}>
                    <i><MdCollectionsBookmark /></i>
                    <span>Collections</span>

                  </Link>
                  <Link to='#'>
                    <i><GiBuyCard /></i>
                    <span>Games</span>
                  </Link>
                  <Link to='#'>
                    <i><RiHandHeartLine /></i>
                    <span>Donations</span>
                  </Link>
                </div>
              </div>
              <div className='side-wrapper'>
                <div className='side-title'>Projects</div>
                <div className='side-menu'>
                  <Link to='#'>
                    <i><GrProjects /></i>
                    <span>Projects</span>
                  </Link>
                </div>
              </div>
              <div className='side-wrapper'>
                <div className='side-title'>More Infos</div>
                <div className='side-menu'>
                  <Link to='#'>
                    <i className='fa fa-folder'></i>
                    <span>Docs</span>
                  </Link>
                  <Link to='#'>
                    <i><DiGithubBadge /></i>
                    <span>Git Hub</span>
                  </Link>
                  <Link to='#'>
                    <i><FcFeedback /></i>
                    <span>FeedBack</span>
                  </Link>

                </div>
              </div>
            </div>
            <div className='main-container'>
              <div className='main-header'>
                <Link className='menu-link-main' to='#'>App Flora</Link>

              </div>
              <div className='content-wrapper'>
                {
                  renderContent()
                }
              </div>
            </div>
          </div>
          <div className='overlay-app'></div>
        </div>
      </main>
    </main>
  );
}
