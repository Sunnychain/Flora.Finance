import React from 'react';
import './notfoun.scss';
export default function NotFound() {
    return(
    <main className="main">
    <nav class="shelf">
  <a class="book home-page">Home page</a>
  <a class="book about-us">About us</a>
  <a class="book contact">Contact</a>
  <a class="book faq">F.A.Q.</a>
  
  <span class="book not-found"></span>
 
  <span class="door left"></span>
  <span class="door right"></span>
</nav>
<h1 className="title">coming soon</h1>
<p></p>

</main>
);
}