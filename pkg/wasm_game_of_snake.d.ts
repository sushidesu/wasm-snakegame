/* tslint:disable */
export enum Cell {
  Blank,
  Head,
  Body,
  Apple,
}
/**
*/
export enum Direction {
  Up,
  Down,
  Right,
  Left,
}
/**
*/
/**
*/
export class Snake {
  free(): void;
}
/**
*/
export class Universe {
  free(): void;
/**
* @param {number} direction 
*/
  tick(direction: number): void;
/**
* @returns {Universe} 
*/
  static new(): Universe;
/**
* @returns {number} 
*/
  width(): number;
/**
* @returns {number} 
*/
  height(): number;
/**
* @returns {number} 
*/
  cells(): number;
/**
* @returns {number} 
*/
  score(): number;
}
