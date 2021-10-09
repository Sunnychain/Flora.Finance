import React from 'react';
import { FiTwitter, FiFacebook, FiInstagram } from "react-icons/fi";
import './footer.scss';
const Footer = () => {
    return ( 
        <main>
            <div className="footer"/>
            <div className="icons">
              <a href="#">
                <FiTwitter size={40}/>
              </a>
              <a href="#">
                <FiFacebook size={40} />
              </a>
              <a href="#">
                <FiInstagram size={40} />  
              </a>                                                                                  
           </div>
        </main> 
    );
}

export default Footer