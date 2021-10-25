import React from 'react';
import { Link } from 'react-router-dom';
import logoAbout from '../../images/flora.svg';
import about from '../../images/tree-planted.png';
import tree from '../../images/nft.png';
import './about.scss';
function About () {
  function switchRouter () {
    window.location.pathname = '/contact';
  }

  return (
        <main classNameName="main-contact">
            <header classNameName="header-main-about">
               <Link to="/"> <img src={logoAbout} width="200px" alt="logo" /></Link>
            </header>
            <div className="container-about-flora">
                <section className="hero" id="hero">
                    <h2 className="hero_header">Flora <span className="light">Network</span></h2>
                    <p className="tagline"><b>Who we are? What do we want? How do we intend to Contribute to the planet?</b></p>
                </section>

                <section className="about" id="about">

                    <h2>Trees of Ogham - Card game</h2>
                    <p className="text_column">

                        NFT's card game, where the NFT's that users already have must reprint to participate in the game. Championships charge per lie card over a period of time and feature prize and prediction market
                    </p>

                    <h2>Tokens</h2>
                    <p className="text_column">
                        FLORA (Main Fungible Token) - Deflationary
                        FUNGI (Fungible Carbon Token) - Inflationary
                        FUNGI+ (Carbon Plus Token) - Inflationary <br />
                        POLEN Token fungible merchan currency mirror of community incentives (capped) - Deflationary
                    </p>
                    <h2>NFTS</h2>
                    <p className="text_column">
                        our nfts represent real trees planted, carry the terrain location and precise coordinates of the tree and the expected amount of carbon benefit.

                    </p>
                </section>

                <div className="gallery">
                    <div className="thumbnail">
                        <h1 className="stats">0000</h1>
                        <h4>Tree Planteds</h4>
                        <p>One line description</p>
                    </div>
                    <div className="thumbnail">
                        <h1 className="stats">0000</h1>
                        <h4>CO2 absorbed</h4>
                        <p>One line description</p>
                    </div>
                    <div className="thumbnail">
                        <h1 className="stats">0000</h1>
                        <h4>Minted Tokens</h4>
                        <p>One line description</p>
                    </div>
                    <div className="thumbnail">
                        <h1 className="stats">0000</h1>
                        <h4>Projects</h4>
                        <p>One line description</p>
                    </div>
                </div>

                <section className="banner">
                    <h2 className="parallax">Carbon Credits</h2>
                    <p className="parallax_description">

                        Carbon credits are used for companies or people who need to contribute to the environment, they are used to reduce the amount of C02 produced
                    </p>
                </section>

                <footer>
                    <article className="footer_column">
                        <h3>ABOUT</h3>
                        <img src={about} alt="" width="400" height="200" className="cards" />
                        <p>
                            The Flora movement seeks new forms of interaction for the preservation of the planet, creating a direct link between the blockchain and the forest and its most active drivers.
                            We see the future of this project, giving nature sustainable ways to regenerate with a protected ecosystem and community.
                            As a way of encouraging the consistent repopulation of destroyed ecosystems and in full agreement with the Carbon-Free plan, we want to empower several Nodes across the globe, opening up ways to tap the treasure. Our goal for this hackathon is going to be developing a NFT card game where the cards are NFTs representing real planted trees using metadata for tracing the status and properties of the trees (including tree description, gps geo-localization, land owner, landlord contract, insurance, etc) . We pretend that the real world trees get insured on the blockchain through a partnership with an existent blockchain insurance (etherrisk).

                        </p>
                    </article>
                    <article className="footer_column">
                        <h3>Green Project Launch Pad
                        </h3>
                        <img src={tree} alt="" width="400" height="200" className="cards" />
                        <p>
                            No launch pads will be able to participate in the crowd of green projects. A project to have a crowloan slot will have to be approved by the foundation. After a project is approved, it will join the crowdloan and stay in this state until the crowdloan time runs out or the financial goal is reached. In this crowdloan, the main currency is retained until the project ends. Note that it is until the project ends and not the crowdloan ends. For a project to finish it will have to be validated by the foundation that the standards have all been met. Upon validation of a project like bulletins bulletins are unlocked and the following rewards will be awarded.

                            - Founder of the Project
                            - Do you receive project funding through the stacking of investors' money? Makes sense ????
                            - Receives carbon credit tokens referring to the positive balance of carbon produced since the project started until it ended.

                            - Receives passive carbon credit tokens referring to carbon reduction to which the project contributes annually.

                            - May receive card nfts

                            - Investor

                            - Receives carbon credit tokens referring to the positive balance of carbon produced since the project started until it ended.

                            - Receives carbon credit tokens related to carbon reduction for the project annually annually.

                            - May receive card nfts

                            land owner

                            - While the contract is valid, you can receive some percentage of the carbon produced on your land

                            - You will have your land reforested for free.
                        </p>
                    </article>
                </footer>
                <section className="footer_banner" id="contact">
                    <p className="hero_header">Got any questions? get in touch.</p>
                    <div className="button-contact-flora" onClick={switchRouter}>Contact</div>
                </section>

            </div>
        </main>

  );
}

export default About;
