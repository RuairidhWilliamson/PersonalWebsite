import React from 'react';
import PropTypes from 'prop-types';
import { NavLink } from 'react-router-dom';

import Pages from 'content/pages';
import Icon from 'components/materialIcon/materialIcon';

import './header.scss';

const Navitem = (props) => (
    <NavLink exact className='nav-item' activeClassName='active' to={props.to} style={{paddingLeft: `${(props.indent)}em`}}>
        <Icon icon={props.materialIcon}/> <h3>{props.label}</h3>
    </NavLink>
);

Navitem.propTypes = {
    to: PropTypes.string,
    label: PropTypes.string,
    active: PropTypes.bool,
    materialIcon: PropTypes.string,
    indent: PropTypes.number,
};

export default class Header extends React.Component {

    renderNavitem(page, indent = 1) {
        return <Navitem
            key={page.label}
            active={this.props.location.pathname === page.path}
            materialIcon={page.material_icon}
            label={page.label}
            to={page.path}
            indent={indent}
        />;
    }

    render() {
        return <div className='nav-bar'>
            {Pages.map(page => this.renderNavitem(page))}
        </div>;
    }
}

Header.propTypes = {
    location: PropTypes.object,
};
