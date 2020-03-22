import React from "react";

export const UserMenu: React.FC = () => {
  return (
    <li className="nav-item dropdown">
      <a
        className="nav-link dropdown-toggle"
        href="#"
        id="navbarDropdown"
        role="button"
        data-toggle="dropdown"
        aria-haspopup="true"
        aria-expanded="false"
      >
        <img src="https://www.gravatar.com/avatar/205e460b479e2e5b48aec07710c08d50?s=30" />
        Graham
      </a>
      <div className="dropdown-menu" aria-labelledby="navbarDropdown">
        <a className="dropdown-item" href="#">
          Profile
        </a>
        <div className="dropdown-divider"></div>
        <a className="dropdown-item" href="#">
          Log Out
        </a>
      </div>
    </li>
  );
};
