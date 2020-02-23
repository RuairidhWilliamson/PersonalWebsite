import React from 'react';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';

import './projectSmall.scss';

const overflow = (text, limit) => `${text.slice(0, limit)}${text.length > limit ? '...' : ''}`;

export default class ProjectSmall extends React.Component {
    renderPreview() {
        if (!this.props.image) {
            return;
        }
        if (this.props.image.type) {
            return this.props.image;
        } else {
            return <img src={this.props.image} alt={this.props.image} className='image'/>;
        }
    }

    render(){
        return (
            <Link
                style={{ animationDelay: `${this.props.delay / 5}s`, visibility: this.props.visible ? 'visible' : 'hidden', position: this.props.visible ? 'inherit' : 'absolute' }}
                className='project-small'
                to={`/${this.props.id}`}
            >
                {this.renderPreview()}
                <div className='right'>
                    <div className='description-container'>
                        <h2 className='title'>{this.props.title}</h2>
                        <div className='date'>{this.props.date}</div>
                        <div className='description'>{overflow(this.props.description, 150)}</div>
                    </div>
                    <div className='tags'>
                        {this.props.tags.map(
                            (tag, index) => <Link
                                style={{animationDelay: `${index / 10 + this.props.delay / 5 + 1}s`}}
                                key={tag}
                                to={{ }}
                                className='tag'
                                onClick={() => this.props.searchTag(tag)}
                            >
                                {tag}
                            </Link>,
                        )}
                    </div>
                </div>
            </Link>
        );
    }
}

ProjectSmall.propTypes = {
    id: PropTypes.string.isRequired,
    title: PropTypes.string.isRequired,
    date: PropTypes.string,
    description: PropTypes.string,
    tags: PropTypes.array,
    image: PropTypes.any,
    history: PropTypes.object.isRequired,
    delay: PropTypes.number,
    visible: PropTypes.bool,
    searchTag: PropTypes.func,
};
