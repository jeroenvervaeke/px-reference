import * as wasm from "px-reference";
import * as React from "react";
import { render } from "react-dom";

const printOptionalId = (value: number | undefined): string =>
  value !== undefined ? value.toString() : "None";

const ReferenceComp = () => {
  const [referenceString, setReferenceString] = React.useState(
    "BA-AAAACD-AAAAAEGF"
  );

  const reference = wasm.parse_reference(referenceString);

  const inputField = (
    <input
      onChange={e => setReferenceString(e.target.value)}
      value={referenceString}
    />
  );

  const referenceOutput =
    reference !== undefined ? (
      <div>
        <div>Reference Type: {reference.reference_type}</div>
        <div>
          CompanySpace Id: {printOptionalId(reference.company_space_id)}
        </div>
        <div>
          AggregateRoot Id: {printOptionalId(reference.aggregate_root_id)}
        </div>
        <div>Revision: {printOptionalId(reference.revision)}</div>
        <div>Object Id: {printOptionalId(reference.object_id)}</div>
      </div>
    ) : (
      <div>"Invalid reference"</div>
    );

  return (
    <div>
      {inputField}
      {referenceOutput}
    </div>
  );
};

render(<ReferenceComp />, document.getElementById("app"));
