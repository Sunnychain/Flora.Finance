import React , { useState } from 'react'
import Logo from '../../images/flora.svg';
import { MenuItens } from './MenuItems';
import './navbar.scss';
const Navbar = () => {

  const [clicked, setClick] = useState(false);
  
  return (
    <nav className="NavBarItems">
      <h1 className="navbar-logo">
        <a href="/">
        <i class="fa fa-tree fa-1x" aria-hidden="true"></i>
          <img src={Logo} />
        </a>
      </h1>
      <ul className={ clicked ? 'nav-menu active' : 'nav-menu' }>
        {
          MenuItens.map((item,index) => {
                //{ window.location.pathname === '/carbon-credits' ?  }
            return (
              <li key={ index }>
               <a className={ item.cName } href={ item.url }>{ item.title }</a>
              </li>
            )
          })
        }
      </ul>
    </nav>
  )
}
export default Navbar;  