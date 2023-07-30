import { useState } from 'react'
import './App.css'
import ParseInstruction from "./parseInstruction"
import axios from 'axios'

function get_parseInstruction(data) {
  switch (data.parser) {
    case 'Selectors': return { "Selectors": [data.selector, { [data.order]: data.index }] }
    case 'Regex': return { "Regex": data.regex }
    case 'None': return null;
  }

}
function App() {
  const [formData, setFormData] = useState({ title: "", url: "", url_suffix: "" });
  let items = ["blog_section", "blog_link_selector", "article_section", "article_headline", "article_date", "article_content"];

  let parseInstructionStates = {};
  for (let i in items) {
    parseInstructionStates[items[i]] = useState({ parser: "Selectors", order: "Normal", selector: "", regex: "", index: 0 });
  }

  const handleChange = (event) => {
    const { name, value } = event.target;
    setFormData((prevFormData) => ({ ...prevFormData, [name]: value }));
  };

  const handleSubmit = (event) => {
    event.preventDefault();

    let payload = {
      'title': formData.title,
      'url': formData.url,
      'url_suffix': formData.url_suffix,
      'index': {
        'section': get_parseInstruction(parseInstructionStates['blog_section'][0]),
        'link_selector': get_parseInstruction(parseInstructionStates['blog_link_selector'][0])
      },
      'article': {
        "section": get_parseInstruction(parseInstructionStates['article_section'][0]),
        "headline": get_parseInstruction(parseInstructionStates['article_headline'][0]),
        "date": get_parseInstruction(parseInstructionStates['article_date'][0]),
        "content": get_parseInstruction(parseInstructionStates['article_content'][0]),
      }
    };
    console.log(payload);
    axios.post('http://localhost:3031/blog', payload)
    .then((response) => {
      console.log(response.data);
        // Handle data
    })
    .catch((error) => {
      console.log(error);
    })
  };

  return (
    <form onSubmit={handleSubmit}>
      <div className='container'>
        <label className='title'>Title:</label>
        <input type="text" id="title" name="title" value={formData.title} onChange={handleChange} />
      </div>
      <div className='container'>
        <label className='title'>Url:</label>
        <input type="text" id="url" name="url" value={formData.url} onChange={handleChange} />
      </div>
      <div className='container'>
        <label className='title'>Url Suffix:</label>
        <input type="text" id="url_suffix" name="url_suffix" value={formData.url_suffix} onChange={handleChange} />
      </div>
      <h2>Blog</h2>
      <ParseInstruction title="Section" states={parseInstructionStates["blog_section"]} />
      <ParseInstruction title="Link Selector" states={parseInstructionStates["blog_link_selector"]} />

      <h2>Article</h2>
      <ParseInstruction title="Section" states={parseInstructionStates["article_section"]} />
      <ParseInstruction title="headline" states={parseInstructionStates["article_headline"]} />
      <ParseInstruction title="date" states={parseInstructionStates["article_date"]} none={true} />
      <ParseInstruction title="content" states={parseInstructionStates["article_content"]} none={true} />
      <button type="submit">Submit</button>
    </form >
  )
}

export default App
