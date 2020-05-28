import React from 'react';
import logo from './logo.svg';
import './App.css';

function App() {

  function handleReadOnlyClick() {
    window.location.href ='/graphiql'
  }

  return (
    <div className="App">
      <header className="App-header">
        <div>
            <h1>Reciclopedia</h1>
            <form action="/login" method="post">
                <label>Username:</label><br/>
                <input type="text" id="username" name="username"/><br/>
                <label>Password:</label><br/>
                <input type="password" id="password" name="password"/><br/>
                <br/>
                <input type="submit" value="Login"/>
                <input type="button" value="Proceed read-only" onClick={handleReadOnlyClick}/>
            </form>
        </div>
        <br/>
        <br/>
        <div>
            <form action="/logout" method="post">
                <input type="submit" value="Logout"/>
            </form>
        </div>
      </header>

    </div>
  );
}

export default App;
