import React from 'react'
import Flippy, { FrontSide, BackSide } from 'react-flippy';
import cardFlora from '../../images/cardImg.png';
import Footer from '../../Components/Footer/Footer';
import './carbonCredits.scss';
const Services = () => {
  return (
    <main>
      <div className="header">
        <div className="content-carbon">
          <h1>Carbon credits</h1>
        </div>
      </div>
      <div className="carbon-contributing">
        <div className="introduction">
          <h1>Contributing carbon</h1>
          <div className="content-p">
          <p>
            We allow our investors to contribute to the environment by purchasing our tokens that are converted into real trees planted to help in carbon production
          </p>
          </div>
        </div>
      </div>
      <div className="agilit-contribuition">
        <h1>Agility in contribution</h1>
        <p>the collaboration process is super fast and easy, with only 3 processes, namely:</p>
      </div>
      <div className="img-proced" />
      <div className="products">
        <h1>product specification</h1>
      </div>
      <div className="flip">
      <Flippy
          flipOnHover={true} 
          flipOnClick={false}
          flipDirection="horizontal" 
          style={{ margin:"0 0  0 10px", width: '380px', height: '400px' }}>
        <FrontSide
          style={{
            backgroundColor: '#6dbb37',
            borderRadius:"20px",
            boxShadow: "10px 10px 5px black"
          }}
        >
        <h2 style={{textAlign: "center", color:"white"}}>Token</h2>
        <img src={cardFlora}/>
        </FrontSide>
        <BackSide
          style={{ backgroundColor: '#6dbb37', borderRadius:"20px", boxShadow: "10px 10px 5px black"}}>
          <h2 style={{textAlign: "center", color:"white"}}>Types:</h2>
          <p style={{fontSize: "20px", color:"white"}}>FLORA TOKEN Deflacionário - Principal</p>
          <p style={{fontSize: "20px", color:"white"}}>FUNGI Token fungível de Carbono - Inflacionário</p>
          <p style={{fontSize: "20px", color:"white"}}>FUNGI+ Token de Carbono Plus - Inflacionário </p>
        </BackSide>
      </Flippy>
      <Flippy
          flipOnHover={true} 
          flipOnClick={false}
          flipDirection="vertical" 
          style={{ margin:"0 0  0 10px", width: '380px', height: '400px' }}>
        <FrontSide
          style={{
            backgroundColor: '#6dbb37',
            borderRadius:"20px",
            boxShadow: "10px 10px 5px black"
          }}
        >
        <h2 style={{textAlign: "center", color:"white"}}>NFT</h2>
        <img src={cardFlora}/>
        </FrontSide>
        <BackSide
          style={{ backgroundColor: '#6dbb37', borderRadius:"25px", boxShadow: "10px 10px 5px black"}}>
          <h2 style={{textAlign: "center", color:"white"}}>Types:</h2>
          <p style={{fontSize: "20px", color:"white"}}>our nfts are based on already planted trees that are already benefiting the environment</p>
        </BackSide>
      </Flippy>
      <Flippy
          flipOnHover={true} 
          flipOnClick={false}
          flipDirection="horizontal" 
          style={{ margin:"0 10px 0 0px", width: '380px', height: '400px', }}>
        <FrontSide
          style={{
            backgroundColor: '#6dbb37',
            borderRadius:"20px",
            boxShadow: "10px 10px 5px black"
          }}
        >
        <h2 style={{textAlign: "center", color:"white"}}>Market</h2>
        <img src={cardFlora}/>
        </FrontSide>
        <BackSide
          style={{ backgroundColor: '#6dbb37',  borderRadius:"20px", boxShadow: "10px 10px 5px black"}}>
          <h2 style={{textAlign: "center", color:"white"}}>Types:</h2>
          <p style={{fontSize: "20px", color:"white"}}>A company or person can buy flora tokens in the market and use them to reduce their carbon defitic</p>
        </BackSide>
      </Flippy>
      </div>
      <Footer />
    </main>
  )
}
export default Services;