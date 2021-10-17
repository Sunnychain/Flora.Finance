import React from 'react';
import { MdCollectionsBookmark } from "react-icons/md";
import { RiHandHeartLine } from "react-icons/ri";
import { GiBuyCard } from "react-icons/gi";
import './Sidebar.scss';
export default function SideBar () {
    return (
        <div className="page-wrapper chiller-theme toggled">
  <a id="show-sidebar" className="btn btn-sm btn-dark" href="#">
    <i className="fas fa-bars"></i>
  </a>
  <nav id="sidebar" className="sidebar-wrapper">
    <div className="sidebar-content">
      <div className="sidebar-brand">
        <a href="#">Flora.Finance</a>
        <div id="close-sidebar">
          <i className="fas fa-times"></i>
        </div>
      </div>
     
  
      <div className="sidebar-search">
        <div>
          <div className="input-group">
            <input type="text" className="form-control search-menu" placeholder="Search..." />
            <div className="input-group-append">
              <span className="input-group-text">
                <i className="fa fa-search" aria-hidden="true"></i>
              </span>
            </div>
          </div>
        </div>
      </div>
     
      <div className="sidebar-menu">
        <ul>
          <li className="header-menu">
            <span>General</span>
          </li>
          <li>
            <a href="#">
              <i className="fa fa-home"></i>
              <span>Home</span>
            </a>
          </li>
          
          <li>
          <li className="sidebar-dropdown">
            <a href="#">
              <i className="fa fa-chart-line"></i>
              <span>Trade</span>
            </a>
            <div className="sidebar-submenu">
              <ul>
                <li>
                  <a href="#">Liquidity</a>
                </li>
                <li>
                  <a href="#">exchange</a>
                </li>
              </ul>
            </div>
          </li>
            <a href="#">
              <i className="fa fa-tractor"></i>
              <span>Farms</span>
            </a>
          </li>
          <li>
            <a href="#">
              <i className="fa fa-calendar"></i>
              <span>Staking</span>
            </a>
          </li>
          
          <li className="sidebar-dropdown">
            <a href="#">
              <i className="fa "><MdCollectionsBookmark/></i>
              <span>Collections</span>
            </a>
            <div className="sidebar-submenu">
              <ul>
                <li>
                  <a href="#">All Collections</a>
                </li>
                <li>
                  <a href="#">Create Collection</a>
                </li>
              </ul>
            </div>
          </li>
          
          <li className="sidebar-dropdown">
            <a href="#">
              <i className="fa "><GiBuyCard/></i>
              <span>Games</span>
            </a>
            <div className="sidebar-submenu">
              <ul>
                <li>
                  <a href="#">Card Game</a>
                </li>
              </ul>
            </div>
            <li>
            <a href="#">
              <i className="fa fa-hearth"><RiHandHeartLine /></i>
              <span>Donations</span>
            </a>
          </li>
          </li>
          <li className="header-menu">
            <span>Extra</span>
          </li>
          <li>
            <a href="#">
              <i className="fa fa-book"></i>
              <span>My NFTree</span>
              <span className="badge badge-pill badge-primary">Beta</span>
            </a>
          </li>
          <li>
            <a href="#">
              <i className="fa fa-calendar"></i>
              <span>My Tokens</span>
            </a>
          </li>
          <li>
            <a href="#">
              <i className="fa fa-folder"></i>
              <span>Auctions</span>
            </a>
          </li>
        </ul>
      </div>
    </div>
    <div className="sidebar-footer">
      <a href="#">
        <i className="fa fa-bell"></i>
        <span className="badge badge-pill badge-warning notification"></span>
      </a>
      <a href="#">
        <i className="fa fa-envelope"></i>
        <span className="badge badge-pill badge-success notification"></span>
      </a>
      <a href="#">
        <i className="fa fa-cog"></i>
        <span className="badge-sonar"></span>
      </a>
      <a href="#">
        <i className="fa fa-power-off"></i>
      </a>
    </div>
  </nav>
  </div>
    );

}