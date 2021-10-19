import React from 'react';
import { Link } from 'react-router-dom';
import './notfoun.scss';
export default function NotFound () {
  return (
    <main className="main">
      <nav class="shelf">
        <Link class="book home-page">Home page</Link>
        <Link class="book about-us">About us</Link>
        <Link class="book contact">Contact</Link>
        <Link class="book faq">F.A.Q.</Link>

        <span class="book not-found"></span>

        <span class="door left"></span>
        <span class="door right"></span>
      </nav>
      <h1 className="title">coming soon</h1>
      <p></p>

    </main>
  );
}
