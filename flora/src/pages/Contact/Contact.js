import React from 'react';
import Header from '../../Components/Header/Header';
import Footer from '../../Components/Footer/Footer';
import './contact.scss';
const Contact = () => {
  return (
    <main>
      <Header/>
      <div className="contact-style">
        <div className="contact-form">
          <h1>Contact Us</h1>
          <p>
            Want to know more about our project and our intentions? or do you have any questions or suggestions for our project? feel free to see us.
          </p>
        </div>
      </div>
      <div className="form">
        <h2>Leave your message here and we will contact you as soon as possible</h2>
      </div>
      <section className="content">
        <div className="feedback-description">
          <h1 className="title"> Have a questions?</h1>
          <p className="subtitle"> Leave your question or Feedback here</p>
        </div> 
        <form className="feedback-form">
        <input placeholder="Email" class="feedback-form__email" required="" />
        <textarea cols="30" class="feedback-form__message" name="text" placeholder="message" rows="5" required="" />
        <button className="feedback-form__submit"> Sumbit</button>
        </form>
       </section>
      <Footer/>
    </main>
  )
}
export default Contact;