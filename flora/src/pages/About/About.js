import React from 'react'
import Footer from '../../Components/Footer/Footer';
import Header from '../../Components/Header/Header';
import './about.scss';
const About = () => {
  return (
    <main>
      <Header/>
      <div className="about">
        <h1>Why FloraNetwork?</h1>
        <h3>what is our focus and what is our project?</h3>
        <p>The Flora movement seeks new forms of interaction for the preservation of the planet, creating a direct link between the blockchain and the forest and its most active drivers.</p>
        <p>We see the future of this project, giving nature sustainable ways to regenerate with a protected ecosystem and community.</p>
      </div>
      <div className="about-into">
        <h1>Flora Project Intentions </h1>
        <h3>intentions for the project</h3>
        <p>We intend to create a treasury in order to support green community projects around the globe. To reach this goal, we need to encourage new projects through our green launchpad, available for projects that meet our sustainability standards. We will keep and track the reputation of the certified land owner through a scalable ranking program.</p>
      </div>
      <div className="game-info">
        <h1>ideas for the future</h1>
        <h3>what do we intend to do with the project in the future?</h3>
        <p>As future work we intend to develop a fungible Carbon Token that reflects the amount of carbon emissions reduced. From the NFTs on the card game, since it represents a real tree, when we implement the carbon credit a NFT holder will be able to claim some carbon tokens that represent the amount of carbon emission reduced by that tree and a new version of the nft will be migrated with the new metadata information. A NFT holder will have carbon tokens rewards for the next 20 years and the first batch reward is available for claiming  after 5 years of buying the NFT. </p>
      </div>
      <Footer />
    </main>
  )
}
export default About;