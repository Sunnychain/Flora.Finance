import { BiCoin } from "react-icons/bi";
import { MDBCard, MDBCardImage, MDBCardBody, MDBCardTitle, MDBCardText, MDBRow, MDBCol } from 'mdb-react-ui-kit';
import Footer from '../../Components/Footer/Footer';
import { GrDocumentPdf } from "react-icons/gr";
import React from 'react';
import tree from '../../images/tree.gif';
import logoFlora from '../../images/logo_flora.png';
import cardFlora from '../../images/cardImg.png';
import logoCard from '../../images/Card.png';
import './Home.scss';

const Home = () => {
  return (
    <main>
      <div className="logo"></div>
        <div className="background-init">
          <h1>Flora Network</h1>
          <h1>eco Project</h1>
        </div>
        <div className="planted-info">
          <p>number of planted trees</p>
          <img src={tree} /> <span> 0 </span>
          <p className="numberPlants">planted Trees</p>
          <div className="plantedInfo">
            <button><GrDocumentPdf/> Whitepaper</button>
            <button><BiCoin /> be a collaborator</button>
          </div>
        </div>
        <section>
          <div className="group-info">
            <h3>Group:</h3>
            <img src={ logoFlora } />
          </div>
        </section>
        <section>
          <div className="intro-invest">
            <div className="box">
              <h1>Investment in a Future</h1>
              <h4>our projects guarantee our investors collaboration in a healthy ecosystem</h4>
            </div>
          </div>
        <div className="flora">
          <h1>Why FloraNetwork?</h1>
          <h3>what is our focus and what is our project?</h3>
          <p>The Flora movement seeks new forms of interaction for the preservation of the planet, creating a direct link between the blockchain and the forest and its most active drivers.</p>
          <p>We see the future of this project, giving nature sustainable ways to regenerate with a protected ecosystem and community.</p>
        </div>
      </section>
      <MDBRow className='row-cols-1 row-cols-md-3 g-3'>
      <MDBCol>
        <MDBCard style={{"border-radius": "10px"}} className="cardAbout">
          <MDBCardTitle style={{margin: "auto"}} >Tokens</MDBCardTitle>
          <MDBCardImage
            src={cardFlora}
            alt='...'
            position='top'
          />
          <MDBCardBody>
            <MDBCardTitle>Description</MDBCardTitle>
            <MDBCardText>
              <p>1 - FLORA (Token fungível principal) - Deflacionário</p>
              <p> 2 - FUNGI (Token fungível de Carbono) - Inflacionário</p> 
              <p> 2 - FUNGI+ (Token de Carbono Plus) - Inflacionário</p> 
              <p> 3 - POLEN Token fungivel de merchan espelho monetario dos incentivos comunitarios (capped) - Deflacionário</p>

            </MDBCardText>
          </MDBCardBody>
        </MDBCard> 
      </MDBCol>
      <MDBCol>
        <MDBCard style={{"border-radius": "10px"}} className="cardAbout">
          <MDBCardTitle style={{margin: "auto"}}>Carbon Credits</MDBCardTitle>
          <MDBCardImage
            src={tree}
            alt='...'
            position='top'
          />
          <MDBCardBody>
            <MDBCardTitle>Description</MDBCardTitle>
            <MDBCardText>
              <p>
              We aim to develop a fungible Carbon Token that reflects the amount of reduced carbon transport. From the NFTs in the card game, by representing a real tree, when we implement the carbon credit, an NFT holder can claim some carbon tokens that represent the amount of carbon emission reduced by that tree and a new version of the NFT will be migrated with the new metadata information
              </p>
            </MDBCardText>
          </MDBCardBody>
        </MDBCard>
      </MDBCol>
      <MDBCol>
        <MDBCard style={{"border-radius": "10px",}} className="cardAbout">
          <MDBCardTitle style={{margin: "auto"}}>Game Cards</MDBCardTitle>
          <MDBCardImage
            src={logoCard}
            alt='...'
            position='top'
          />
          <MDBCardBody>
            <MDBCardTitle>Description</MDBCardTitle>
            <MDBCardText>
              <p>our card game has its rarity based on types of trees planted in real life</p>
              <p>::Carbon NFT:: Growth cards (arvores autoctones)</p>
              <p>:: NFT Raros:: Active cards (árvores de vagas)</p>
              <p>::Nfts Muito raros:: Active cards (árvores de fruta)</p>
              <p>::Nfts Lendaria::Active cards (arvores ancestrais)</p>
            </MDBCardText>
          </MDBCardBody>
        </MDBCard>
      </MDBCol>
    </MDBRow>
    <Footer />
  </main>
  )
}
export default Home;
