use bitflags::bitflags;

#[derive(Clone, Copy)]
#[allow(non_camel_case_types, dead_code)]
pub enum Register {
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

bitflags! {
    #[allow(non_camel_case_types)]
    pub struct CTRL3_CBits: u8 {
        const BOOT = 0b1000_0000;
        const BDU = 0b0100_0000;
        const H_LACTIVE = 0b0010_0000;
        const PP_OD = 0b0001_0000;
        const SIM = 0b0000_1000;
        const IF_INC = 0b0000_0100;
        const BIG_ENDIAN = 0b0000_0010;
        const SW_RESET = 0b0000_0001;
    }
}

bitflags! {
    #[allow(non_camel_case_types)]
    pub struct CTRL1_XLBits: u8 {
        /// Accelerometer Scale
        const Scale2G  = 0b0000_0000;
        const Scale4G  = 0b0000_1000;
        const Scale8G  = 0b0000_1100;
        const Scale16G = 0b0000_0100;

        /// Accelerometer Output Data Rates (Hz)
        const ODR12_5Hz  = 0b0001_0000;
        const ODR26Hz    = 0b0010_0000;
        const ODR52Hz    = 0b0011_0000;
        const ODR104Hz   = 0b0100_0000;
        const ODR208Hz   = 0b0101_0000;
        const ODR416Hz   = 0b0110_0000;
        const ODR833Hz   = 0b0111_0000;
        const ODR1_66Khz = 0b1000_0000;
        const ODR3_33Khz = 0b1001_0000;
        const ODR6_66Khz = 0b1010_0000;
    }
}

bitflags! {
    #[allow(non_camel_case_types)]
    pub struct CTRL2_GBits: u8 {
        /// Gyroscope Full Scale Detection (degrees/s)
        const FS125DPS  = 0b0000_0010;
        const FS250DPS  = 0b0000_0000;
        const FS500DPS  = 0b0000_0100;
        const FS1000DPS = 0b0000_1000;
        const FS2000DPS = 0b0000_1100;

        /// Gyroscope Output Data Rates (Hz)
        const ODR12_5Hz  = 0b0001_0000;
        const ODR26Hz    = 0b0010_0000;
        const ODR52Hz    = 0b0011_0000;
        const ODR104Hz   = 0b0100_0000;
        const ODR208Hz   = 0b0101_0000;
        const ODR416Hz   = 0b0110_0000;
        const ODR833Hz   = 0b0111_0000;
        const ODR1_66Khz = 0b1000_0000;
        const ODR3_33Khz = 0b1001_0000;
        const ODR6_66Khz = 0b1010_0000;
    }
}
