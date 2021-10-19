import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
import Flora from '../../images/flora.svg';
import $ from 'jquery';
import '../globalStyle.scss';

export default function Content () {
  function research () {
    const dropdowns = document.querySelectorAll('.dropdown');
    dropdowns.forEach((dropdown) => {
      dropdown.addEventListener('click', (e) => {
        e.stopPropagation();
        dropdowns.forEach((c) => c.classNameList.remove('is-active'));
        dropdown.classNameList.add('is-active');
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
      document.body.classNameList.toggle('light-mode');
      console.log('narguilada');
    });
  }

  useEffect(() => {
    research();
  }, []);
  return (

    <main className="page-content">
      <img src={Flora} width="220px" alt="flora" />
      <div className="container">
        <h3>Contribute to the planet through projects that encourage the preservation of the environment</h3>

        <div className="row">
          <div className="form-group col-md-12">
            <p>Flora is a project that aims to contribute to the planet by providing companies or people with ways to reduce the amount of carbon generated</p>
            <p> Our NFTS represent real trees planted on our planet, with them come their proper coordinates and a type of card for our game</p>
          </div>
          <div className="form-group col-md-12 ">
            <div className="contentAbout">

            </div>
          </div>

        </div>

        <h5>in Comming</h5>

        <div className="row">
          <div className="col-xs-12 col-sm-3 col-md-3 col-lg-3">
            <div className="card rounded-0 p-0 shadow-sm box-img ">

              <div className="card-body text-center">
                <h3 className="card-title">Farms & Staking</h3>
                <h4>Flora to Harvest:</h4>
                <h4>~$0.00</h4>
                <h4>Flora in Wallet</h4>
                <h4>Locked</h4>
                <Link to="#" target="_blank" className="btn btn-primary btn-lg" rel="noreferrer">View</Link>
                <Link to="#" target="_blank" className="btn btn-success btn-lg" rel="noreferrer">About</Link>

              </div>

            </div>
          </div>
          <div className="col-xs-12 col-sm-5 col-md-5 col-lg-5">
            <div className="card rounded-0 p-0 shadow-sm box-img">

              <div className="card-body text-center">
                <h3 className="card-title">FLORA Deflationary</h3>
                <h4> Total Minted: <b>0</b></h4>
                <h4>Total Burned <b>0</b></h4>
                <h4>New Blocks <b>0</b></h4>
                <h4>Next Emission Reduction </h4>
                <Link to="#" target="_blank" className="btn btn-primary btn-lg" rel="noreferrer">View</Link>

              </div>

            </div>
          </div>
          <div className="col-xs-12 col-sm-4 col-md-4 col-lg-4">
            <div className="card rounded-0 p-0 shadow-sm box-img">

              <div className="card-body text-center">
                <h3 className="card-title"> POLEN capped</h3>
                <h4> Total Minted: <b>0</b></h4>
                <h4>Total Burned <b>0</b></h4>
                <h4>New Blocks <b>0</b></h4>
                <h4>Next Emission Reduction </h4>
                <Link to="#" target="_blank" className="btn btn-primary btn-lg" rel="noreferrer">View</Link>

              </div>

            </div>
          </div>
        </div>
        <div className="row" style={{ marginTop: '15px' }}>
          <div className="col-xs-12 col-sm-12 col-md-12 col-lg-12">
            <div className="card rounded-0 p-0 shadow-sm box-img">

              <div className="card-body text-center">
                <h1 className="card-title"> Next Battle In:</h1>
                <h4><b>00:00</b> </h4>
                <Link to="#" target="_blank" className="btn btn-primary btn-lg" rel="noreferrer">Go</Link>

              </div>

            </div>
          </div>
        </div>

        <footer className="text-center">

        </footer>

      </div>
    </main>

  );
}
