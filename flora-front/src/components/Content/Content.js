import React from 'react';

import Flora from '../../images/flora.svg';
import './Content.scss';

export default function Content () {
    return(
        <main className="page-content">
    <div className="container">
      <h2>Flora.Finance</h2>
  
      <div className="row">
        <div className="form-group col-md-12">
          <p>This is a responsive sidebar template with dropdown menu based on bootstrap 4 framework.</p>
          <p> You can find the complete code on <a href="https://github.com/azouaoui-med/pro-sidebar-template" target="_blank">
              Github</a>, it contains more themes and background image option</p>
        </div>
       
        <div className="form-group col-md-12">
          <div className="alert alert-success" role="alert">
            <h4 className="alert-heading">New !</h4>
            <p>New react pro sidebar library is now available on <a href="https://www.npmjs.com/package/react-pro-sidebar" target="_blank">npm</a> <a href="https://github.com/azouaoui-med/react-pro-sidebar" target="_blank">
                <img alt="GitHub stars" src="https://img.shields.io/github/stars/azouaoui-med/react-pro-sidebar?style=social" />
              </a></p>
            <a href="https://github.com/azouaoui-med/react-pro-sidebar" target="_blank" className="btn btn-sm btn-primary mr-2">
              Github</a>
            <a href="https://azouaoui-med.github.io/react-pro-sidebar" target="_blank" className="btn btn-sm btn-success">
              Demo</a>

          </div>

        </div>
      </div>
      <h5>Em Andamento</h5>
  
      <div className="row">
        <div className="col-xs-12 col-sm-6 col-md-6 col-lg-6">
          <div className="card rounded-0 p-0 shadow-sm">
		  <h1 style={{textAlign: 'center'}}>Farms & Staking:</h1>
		  <h3 style={{textAlign: 'center'}}>Qnt to Harvest: <b>Locked</b></h3>
            <img src={Flora} className=" card-img-top rounded-0" alt="nft img" />
            <div className="card-body text-center">
              <h6 className="card-title"><b>end-in </b> 00:00</h6>
              <a href="#" target="_blank" className="btn btn-primary btn-sm">Unlock Wallet</a>

            </div>

          </div>
        </div>
        <div className="col-xs-12 col-sm-6 col-md-6 col-lg-6">
          <div className="card rounded-0 p-0 shadow-sm">
		  <h2 style={{textAlign: 'center'}}>Next Round In:</h2>
		  <h1 style={{textAlign: 'center'}}>Next Round In: 00:00</h1>
            <img src={Flora} className="card-img-top rounded-0" alt="flora" />
            <div className="card-body text-center">
              <h6 className="card-title">Next Round In:</h6>
              <a href='#' target="_blank" className="btn btn-success btn-sm">Prepare To Battle</a>
        
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>


    );
}