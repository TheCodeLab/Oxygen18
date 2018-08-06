import * as React from 'react';
import './SafeHtmlDoc.css';

export type SafeHtmlDocProps = {
  node: Node,
}

class SafeHtmlDoc extends React.Component<SafeHtmlDocProps> {
  render(): JSX.Element|null {
    const node = this.props.node;
    const children = [];
    for (var i = 0; i < node.childNodes.length; i++) {
      children.push(
        <SafeHtmlDoc key={i} node={node.childNodes.item(i)} />
      )
    }
    if (node instanceof HTMLElement) {
      switch (node.nodeName) {
        case 'HTML':
        case 'BODY':
          return (
            <React.Fragment>
              {children}
            </React.Fragment>
          );
        case 'HEAD':
          return null;
        case 'A':
          return (
            <a
              className='SafeHtmlDoc-a'
              href={node.getAttribute("href") || undefined}>
              {children}
            </a>
          );
        case 'B':
          return (
            <b>
              {children}
            </b>
          );
        case 'P':
          return (
            <p>
              {children}
            </p>
          );
        case 'UL':
          return (
            <ul>
              {children}
            </ul>
          );
        case 'LI':
          return (
            <li>
              {children}
            </li>
          );
        case 'CODE':
          return (
            <code className='SafeHtmlDoc-code'>
              {children}
            </code>
          );
        case 'H2':
          return (
            <h2 className='SafeHtmlDoc-h2'>
              {children}
            </h2>
          );
        default:
          console.log("Unsupported element", node.nodeName);
      }
    }
    if (node instanceof Text) {
      if (node.textContent && node.textContent.trim().length > 0) {
        return (
          <React.Fragment>
            {node.textContent}
          </React.Fragment>
        );
      }
      else {
        return null;
      }
    }

    return (
      <React.Fragment>
        {children}
      </React.Fragment>
    );
  }
}

export default SafeHtmlDoc;
