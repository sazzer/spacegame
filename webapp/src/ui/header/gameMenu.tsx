import React from "react";

export const GameMenu: React.FC = () => {
  return (
    <>
      <li className="nav-item active">
        <a className="nav-link" href="#">
          System Overview
        </a>
      </li>
      <li className="nav-item">
        <a className="nav-link" href="#">
          Galaxy View
        </a>
      </li>
      <li className="nav-item">
        <a className="nav-link" href="#">
          Fleet View
        </a>
      </li>
      <li className="nav-item">
        <a className="nav-link" href="#">
          Research
        </a>
      </li>
      <li className="nav-item">
        <a className="nav-link" href="#">
          Ship Designer
        </a>
      </li>
    </>
  );
};
