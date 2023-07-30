import { useState } from 'react'

export default function ParseInstruction({ title, none, states }) {
  const formData = states[0];
  const setFormData = states[1];
  none = !!none;
  const handleChange = (event) => {
    const { name, value } = event.target;
    setFormData({ ...formData, [name]: value });
  };

  return (
    <div className="wrapper">
      <div className="container">

        <label className='title'>{title} Parser:</label>
        <select name="parser" value={formData.parser} onChange={handleChange}>
          {none &&
            <option value="None">None</option>
          }
          <option value="Selectors">Selectors</option>
          <option value="Regex">Regex</option>
        </select>


        {formData.parser == "Selectors" ? (
          <div>
            <label>Selector:</label>
            <input type="text" name="selector" value={formData.selector} onChange={handleChange} />
            <label>Order:</label>
            <select name="order" value={formData.order} onChange={handleChange}>
              <option value="Normal">Normal</option>
              <option value="Reverse">Reverse</option>
            </select>

            <label>Index:</label>
            <input type="number" name="index" className="index" value={formData.index} onChange={handleChange} />

          </div>
        ) : (
          formData.parser == "Regex" ? (
            <div>
              <label>Regex:</label>
              <textarea name="regex" value={formData.regex} onChange={handleChange} />
            </div>
          ) : (
            <label></label>
          ))}
        <br />
      </div>
    </div>
  );
}