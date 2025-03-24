import { RiskLevels } from "../backend/backend";
import { createEffect, Show } from "solid-js";

type Props = {
  risk?: RiskLevels
};

const RiskNotifier = (props: Props) => {

  createEffect(() => {
    console.log('The risk is: ', props.risk)
  })

  return (
    <div class="notification-container">
      <Show when={props.risk}>
        <div class={`notification ${props.risk?.toLowerCase()}-risk`}>
          {props.risk} risk
        </div>
      </Show>
    </div>
  );
};

export default RiskNotifier;
