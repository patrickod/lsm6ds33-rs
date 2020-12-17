#[allow(non_camel_case_types, dead_code)]
pub(crate) enum Register {
    // Self ID
    WHO_AM_I   = 0x0F,

    /// Embedded functions configuration
    FUNC_CFG_ACCESS = 0x01,

    /// FIFO control
    FIFO_CTRL1 = 0x06,
    FIFO_CTRL2 = 0x07,
    FIFO_CTRL3 = 0x08,
    FIFO_CTRL4 = 0x09,
    FIFO_CTRL5 = 0x0A,

    ORIENT_CFG_G = 0x0B,

    // Interrupt pin controls
    INT1_CTRL = 0x0D,
    INT2_CTRL = 0x0E,

    /// Accelerometer & Gyroscope controls
    CTRL1_XL = 0x10,
    CTRL2_G  = 0x11,
    CTRL3_C  = 0x12,
    CTRL4_C  = 0x13,
    CTRL5_C  = 0x14,
    CTRL6_C  = 0x15,
    CTRL7_G  = 0x16,
    CTRL8_XL = 0x17,
    CTRL9_XL = 0x18,
    CTRL10_C = 0x19,

    /// Interrupts & Functions
    WAKE_UP_SRC = 0x1B,
    TAP_SRC = 0x1C,
    D6D_SRC = 0x1D,

    /// Status
    STATUS_REG = 0x1E,

    /// Temperature output
    OUT_TEMP_L = 0x20,
    OUT_TEML_H = 0x21,

    /// Gyroscope output
    OUT_X_L_G = 0x22,
    OUT_X_H_G = 0x23,
    OUT_Y_L_G = 0x24,
    OUT_Y_H_G = 0x25,
    OUT_Z_L_G = 0x26,
    OUT_Z_H_G = 0x27,

    /// Accelerometer output
    OUT_X_L_XL = 0x28,
    OUT_X_H_XL = 0x29,
    OUT_Y_L_XL = 0x2A,
    OUT_Y_H_XL = 0x2B,
    OUT_Z_L_XL = 0x2C,
    OUT_Z_H_XL = 0x2D,

    /// FIFO status
    FIFO_STATUS_1 = 0x3A,
    FIFO_STATUS_2 = 0x3B,
    FIFO_STATUS_3 = 0x3C,
    FIFO_STATUS_4 = 0x3D,

    /// FIFO data output
    FIFO_DATA_OUT_L = 0x3E,
    FIFO_DATA_OUT_H = 0x3F,

    /// Timestamp output
    TIMESTAMP0_REG = 0x40,
    TIMESTAMP1_REG = 0x41,
    TIMESTAMP2_REG = 0x42,

    /// Step counter timestamp registers
    STEP_TIMESTAMP_L = 0x49,
    STEP_TIMESTAMP_H = 0x4A,

    /// Interrupt register
    FUNC_SRC = 0x53,

    TAP_CFG = 0x58,
    TAP_THS_6D = 0x59,
    INT_DUR2 = 0x5A,
    WAKE_UP_THS = 0x5B,
    WAKE_UP_DUR = 0x5C,
    FREE_FALL = 0x5D,
    MD1_CFG = 0x5E,
    MD2_CFG = 0x5F,
}

impl Register {
    pub fn address(self) -> u8 {
        self as u8
    }
}
