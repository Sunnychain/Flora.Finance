import { BiCoin } from "react-icons/bi";
import { GrDocumentPdf } from "react-icons/gr";
import Footer from '../../Components/Footer/Footer';
import co2 from '../../images/tree.gif';
import React from 'react'
import './projects.scss'
const Projects = () => {
  return (
   <main>
      <header>
        <h1>
          <div>P</div>
          <div>r</div>
          <div>o</div>
          <div>j</div>
          <div>e</div>
          <div>c</div>
          <div>e</div>
          <div>c</div>
          <div>t</div>
          <div>s</div>
        </h1>
        <div className="box-end">
          <h2>Flora projects around the world</h2>
        </div>
    </header>
    <div className="projects">
      <h2>projetos</h2>
      <p>carbon projects generate token flora for investors</p>
      <div className="planted_info">
        <p>Total Tree Planted</p>
        <img src={co2} /> <span> 0 </span>
        <p className="numberPlants">planted Trees</p>
        <div className="plantedInfo">
          <button><GrDocumentPdf/> Whitepaper</button>
          <button><BiCoin /> be a collaborator</button>
        </div>
      </div>
    </div>
    <Footer />
   </main>
  )
}
export default Projects;