export function useStringColor() {
  function getGradientFromString(value: string): string {
    const hash = getStringHash(value);
    const hue = hash % 360;
    const saturation = 48 + ((hash >>> 8) % 18);
    const lightness = 42 + ((hash >>> 16) % 12);
    const color = hslToHex(hue, saturation, lightness);
    const lightColor = hslToHex(hue, saturation, lightness + 10);

    return `linear-gradient(135deg, ${lightColor}, ${color})`;
  }

  return {
    getGradientFromString,
  };
}

function getStringHash(value: string): number {
  let hash = 2_166_136_261;

  for (const symbol of value) {
    hash ^= symbol.codePointAt(0)!;
    hash = Math.imul(hash, 16_777_619);
  }

  return hash >>> 0;
}

function hslToHex(hue: number, saturation: number, lightness: number): string {
  const chroma = (1 - Math.abs(2 * lightness / 100 - 1)) * saturation / 100;
  const huePrime = hue / 60;
  const secondLargestComponent = chroma * (1 - Math.abs(huePrime % 2 - 1));
  const lightnessMatch = lightness / 100 - chroma / 2;
  let red = 0;
  let green = 0;
  let blue = 0;

  if (huePrime < 1) {
    red = chroma;
    green = secondLargestComponent;
  } else if (huePrime < 2) {
    red = secondLargestComponent;
    green = chroma;
  } else if (huePrime < 3) {
    green = chroma;
    blue = secondLargestComponent;
  } else if (huePrime < 4) {
    green = secondLargestComponent;
    blue = chroma;
  } else if (huePrime < 5) {
    red = secondLargestComponent;
    blue = chroma;
  } else {
    red = chroma;
    blue = secondLargestComponent;
  }

  return `#${[red, green, blue]
    .map((color) => Math.round((color + lightnessMatch) * 255))
    .map((color) => color.toString(16).padStart(2, '0'))
    .join('')}`;
}
