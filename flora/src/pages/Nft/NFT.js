import React from 'react';
import Footer from '../../Components/Footer/Footer';
import './nft.scss'

const Nft = () => {
  return (
    <main>
      <div className="header-nft"></div>
      <div className="nftintro">
        <h1>NFT real Tree</h1>
        <p>Nfts based on real trees that are planted in different regions around the world</p>
        <h3>Location on Map</h3>
        <h5>we give our creditors full assurance of a true and reliable product.</h5>
        <div className="nft-market-btn">
          <h1>check our nfts</h1>
          <button>NFT MARKET</button>
        </div>  
      </div>
      <Footer/>
    </main>
  )
}
export default Nft;