// Units of measurements
//
// pt - point - physical unit: 1/72 inch.
// sp - scaled point - 65536 sp -> 1pt  (1/2^16).
// em - An em is a unit in the field of typography, 
//      equal to the currently specified point size.
//      For example, one em in a 16-point typeface is 16 points.
// 
// DesignUnits - Specifies the font metric to be converted to device units. 
//               This value can be any font metric, including the width of a 
//               character or the ascender value for an entire font.
//
// DeviceUnits - Specifies the DesignUnits font metric converted to device units. 
//               This value is in the same units as the value specified for DeviceResolution.
//
// DeviceResolution - Specifies number of device units (pixels) per inch. 
//                    Typical values might be 300 for a laser printer or 96 for a VGA screen.