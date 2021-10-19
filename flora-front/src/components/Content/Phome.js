import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { MdOutlineGeneratingTokens } from 'react-icons/md';
import { GiCardPickup } from 'react-icons/gi';
import { RiAuctionLine } from 'react-icons/ri';
import $ from 'jquery';
import CO2 from '../../images/co2-1-02.svg';
import logo from '../../images/logo_flora.png';

export default function Phome () {
  function research () {
    const dropdowns = document.querySelectorAll('.dropdown');
    dropdowns.forEach((dropdown) => {
      dropdown.addEventListener('click', (e) => {
        e.stopPropagation();
        dropdowns.forEach((c) => c.classList.remove('is-active'));
        dropdown.classList.add('is-active');
      });
    });

    $('.search-bar input')
      .focus(function () {
        $('.header').addClass('wide');
      })
      .blur(function () {
        $('.header').removeClass('wide');
      });

    $(document).click(function (e) {
      const container = $('.status-button');
      const dd = $('.dropdown');
      if (!container.is(e.target) && container.has(e.target).length === 0) {
        dd.removeClass('is-active');
      }
    });

    $(function () {
      $('.dropdown').on('click', function (e) {
        $('.content-wrapper').addClass('overlay');
        e.stopPropagation();
      });
      $(document).on('click', function (e) {
        if ($(e.target).is('.dropdown') === false) {
          $('.content-wrapper').removeClass('overlay');
        }
      });
    });

    $(function () {
      $('.status-button:not(.open)').on('click', function (e) {
        $('.overlay-app').addClass('is-active');
      });
      $('.pop-up .close').click(function () {
        $('.overlay-app').removeClass('is-active');
      });
    });

    $('.status-button:not(.open)').click(function () {
      $('.pop-up').addClass('visible');
    });

    $('.pop-up .close').click(function () {
      $('.pop-up').removeClass('visible');
    });

    const toggleButton = document.querySelector('.dark-light');
    $(toggleButton).on('click', function () {
      document.body.classList.toggle('light-mode');
      console.log('narguilada');
    });
  }

  useEffect(() => {
    research();
  }, []);
  return (

    <main>
      <div className='content-wrapper-header'>
        <div className='content-wrapper-context'>
          <h3 className="img-content">
            <img src={CO2} alt="c02" />
            Flora is a project that aims to contribute to the planet by providing companies or people with ways to reduce the amount of carbon generated
          </h3>
          <div className='content-text'>Our NFTS represent real trees planted on our planet, with them come their proper coordinates and a type of card for our game</div>
          <button className='content-button'>
            know more</button>
        </div>
        <img className='content-wrapper-img' src={logo} alt='' />
      </div>
      <div className='content-section'>
        <div className='content-section-title'>In Preparation / Ongoing</div>
        <ul>
          <li className='adobe-product'>
            <div className='products'>
            <i className='fa fa-tractor'></i>
              Farms & Staking
            </div>
            <span className='status'>
              <span className='status-circle green'></span>
              Current</span>
            <div className='button-wrapper'>
              <button className='content-button status-button'>Go</button>
              <div className='menu'>
                <button className='dropdown'>
                  <ul>
                    <li><Link to='#'>Staking</Link></li>
                    <li><Link to='#'>Farm Status</Link></li>
                    <li><Link to='#'>Learn More</Link></li>
                  </ul>
                </button>
              </div>
            </div>
          </li>
          <li className='adobe-product'>
            <div className='products'>
              <i><GiCardPickup /></i>
              Battle
            </div>
            <span className='status'>
              <span className='status-circle green'></span>
              Current</span>
            <div className='button-wrapper'>
              <button className='content-button status-button'>Go</button>
              <div className='menu'>
                <button className='dropdown'>
                  <ul>
                    <li><Link to='#'>Tutorial</Link></li>
                    <li><Link to='#'>View Next Battle</Link></li>
                    <li><Link to='#'>My cards</Link></li>
                  </ul>
                </button>
              </div>
            </div>
          </li>
          <li className='adobe-product'>
            <div className='products'>
              <i><RiAuctionLine /></i>
              Auctions
            </div>
            <span className='status'>
              <span className='status-circle green'></span>
              Current</span>
            <div className='button-wrapper'>
              <button className='content-button status-button'>Go</button>
              <div className='menu'>
                <button className='dropdown'>
                  <ul>
                    <li><Link to='#'>View All Auctions</Link></li>
                    <li><Link to='#'>Create Auctions</Link></li>
                    <li><Link to='#'>Top hated</Link></li>
                  </ul>
                </button>
              </div>
            </div>
          </li>
        </ul>
      </div>
      <div className='content-section'>
        <div className='content-section-title'>Info</div>
        <div className='apps-card'>
          <div className='app-card'>
            <span>
            <i className='fa fa-tractor'></i>
              Farms & Staking
            </span>
            <div className='app-card__subtext'></div>
            <p>Flora to Harvest:</p>
            <p>~$0.00</p>
            <p>Flora in Wallet</p>
            <p>Locked</p>
           <div className='app-card-buttons'>
              <button className='content-button status-button'>View All</button>
              <div className='menu'></div>
            </div>
          </div>
          <div className='app-card'>
            <span>
              <MdOutlineGeneratingTokens size={50}/>
              <b>FLORA Deflationary</b>
            </span>
            <div className='app-card__subtext'>
              <p> Total Minted: <b>0</b></p>
              <p>Total Burned <b>0</b></p>
              <p>New Blocks <b>0</b></p>
              <p>Next Emission Reduction </p>
            </div>
            <div className='app-card-buttons'>
              <button className='content-button status-button'>View</button>
              <div className='menu'></div>
            </div>
          </div>
          <div className='app-card'>
            <span>
            <MdOutlineGeneratingTokens size={50}/>
              POLEN capped
            </span>
            <div className='app-card__subtext'>
            <p> Total Minted: <b>0</b></p>
              <p>Total Burned <b>0</b></p>
              <p>New Blocks <b>0</b></p>
              <p>Next Emission Reduction </p>
            </div>
            <div className='app-card-buttons'>
              <button className='content-button status-button'>View</button>
              <div className='menu'></div>
            </div>
          </div>
        </div>
      </div>
    </main>

  );
}
