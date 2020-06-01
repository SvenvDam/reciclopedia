import React from 'react';
import './App.css';
import Cookies from 'js-cookie'

function App() {
  return (
    <div className="App">
      <Nav />
      <Link />
    </div>
  );
}

export default App;

function Nav() {
  return (
    <div>
      <ul className="Nav">
        <li className="Title">Reciclopedia</li>
        <li><User /></li>
      </ul>
    </div>
  )
}

function User() {
  const cookieValue = Cookies.get("User-Session-Token")
  if (cookieValue) {
    const displayName = "Hi, " + cookieValue.split("##")[0]

    return (
      <form action="/logout" method="post" className="User">
        <label>{displayName}</label>
        <input type="submit" value="Logout" />
      </form>
    )
  } else {
    return (
      <form action="/login" method="post" className="User">
        <input type="text" id="username" name="username" placeholder="Username" />
        <input type="password" id="password" name="password" placeholder="Password" />
        <input type="submit" value="Login" />
      </form>
    )
  }
}

function Link() {
  return (
    <div className="Link">
      <button onClick={() => window.location.href = "/graphiql"}>
        <h1>Graphiql</h1>
    </button>
    </div>
  )
}