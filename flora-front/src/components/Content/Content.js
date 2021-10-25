import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
import Flora from '../../images/flora.svg';
import $ from 'jquery';
import '../globalStyle.scss';

export default function Content () {
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
          <div className="col-xs-12 col-sm-12 col-md-12 col-lg-12"
          style={{ marginBottom: '25px' }}>
            <div className="card rounded-0 p-0 shadow-sm box-img farms">
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
          <div className="col-xs-12 col-sm-6 col-md-6 col-lg-6">
            <div className="card rounded-0 p-0 shadow-sm box-img flora">
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
          <div className="col-xs-12 col-sm-6 col-md-6 col-lg-6">
            <div className="card rounded-0 p-0 shadow-sm box-img capped">

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
              <div className="card-body text-center oghan">
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
