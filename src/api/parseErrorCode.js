import { ref, watch } from 'vue'

const faultBits = [
  "overVoltage",
  "underVoltage",
  "motorOverTemp",
  "moduleOverTemp",
  "moduleOverCurrent",
  "overPeakCurrent",
  "overLoad",
  "motorLostPhase",
  "currentUnbalance",
  "motorStall",
  "startupFailed",
  "overSpeed",
  "reserve12",
  "reserve13",
  "currentOffset",
  "voltageOffset",
]

function getFaultStates(errorCode) {
  const result = [];
  for (let i = 0; i < faultBits.length; i++) {
    if ((errorCode & (1 << i)) !== 0) {
      result.push({ flag: faultBits[i]});
    }
  }
  return result;
}

export default {
  getFaultStates,
}
