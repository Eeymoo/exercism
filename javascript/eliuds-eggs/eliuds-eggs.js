//
// This is only a SKELETON file for the 'Eliud's Eggs' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const eggCount = (displayValue) => {
  let eggCount = 0;
  
  while (displayValue !== 0) { 
    if (displayValue & 1) {
      eggCount++;
    }
    displayValue = displayValue >> 1;
  }
  return eggCount;
};
