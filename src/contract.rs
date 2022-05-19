#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, 
  Empty, Response, StdResult, StdError};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, PixelResponse, ColorsResponse};
use crate::state::{State, STATE, CanvasState, COLORS, PixelData, PIXELS, COL_PIXELS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:canvas";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    let colors = CanvasState {
        colors: vec![16777215; 25] // 16777215 is equivalent to #FFFFFF (white)
    };
    COLORS.save(deps.storage, &colors)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BuyPixel { x, y, color, price,
          for_sale, message } => try_buy(deps, info, x, y, color, price, for_sale, message),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

pub fn try_buy(deps: DepsMut, info: MessageInfo, x: u32, y: u32, color: u32, price: u64, for_sale: bool, message: String) -> Result<Response, ContractError> {
    let update_pixel = |pixel_data: Option<PixelData>| -> StdResult<PixelData> {
      match pixel_data {
        Some(_) => Err(StdError::generic_err("Pixel data already exists")),
        None => Ok(PixelData {
            owner: info.sender.to_string(),
            price: price,
            for_sale: for_sale,
            message: message
        })
      }
    };

    let update_color = |mut canvas_state: CanvasState| -> StdResult<CanvasState> {
        canvas_state.colors[get_index_from_coords(x, y) as usize] = color;
        Ok(canvas_state)
    };

    PIXELS.update(deps.storage, get_index_from_coords(x, y).to_string(), update_pixel)?;
    COLORS.update(deps.storage, update_color)?;
    Ok(Response::new().add_attribute("method", "try_buy"))
}

pub fn get_color(deps: Deps, x: u32, y: u32) -> StdResult<u32> {
    let values = COLORS.load(deps.storage)?.colors;
    return Ok(values[get_index_from_coords(x, y) as usize])
}

pub fn get_pixel(deps: Deps, x: u32, y: u32) -> StdResult<PixelData> {
    return PIXELS.load(deps.storage, get_index_from_coords(x, y).to_string())
}

pub fn get_colors(deps: Deps) -> StdResult<Vec<u32>> {
    let values = COLORS.load(deps.storage)?.colors;
    return Ok(values);
}

fn get_index_from_coords(x: u32, y: u32) -> u32 {
    // Top left is (0, 0). Bottom right is (COL_PIXELS, ROW_PIXELS)
    return y * COL_PIXELS + x;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPixel {x, y} => {
          return to_binary(&PixelResponse{
            x: x,
            y: y,
            color: get_color(deps, x, y)?,
            pixel_data: get_pixel(deps, x, y)?
          })
        },
        QueryMsg::GetColors {} => return to_binary(&ColorsResponse{
          colors: get_colors(deps)?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_dependencies_with_balances, mock_env, mock_info,
      MockApi, MockQuerier};
    use cosmwasm_std::{from_binary, coins, OwnedDeps, MemoryStorage};

    const MILLION: u64 = 1000000;

    fn instantiate_contract(deps: &mut OwnedDeps<MemoryStorage, MockApi, MockQuerier>) {
      let msg = InstantiateMsg {};
      let info = mock_info("creator", &coins(1000, "ujunox"));

      // Instantiate the contract
      let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
      assert_eq!(0, res.messages.len());
    }

    #[test]
    fn get_index_from_coords_test() {
        assert_eq!(0, get_index_from_coords(0, 0));
        assert_eq!(9, get_index_from_coords(4, 1));
        // assert_eq!(100, get_index_from_coords(0, 1));
        // assert_eq!(104, get_index_from_coords(4, 1));
        // assert_eq!(200, get_index_from_coords(0, 2));
    }

    #[test]
    fn initialization_test() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "ujunox"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetColors {}).unwrap();
        let price_response: ColorsResponse = from_binary(&res).unwrap();
        assert_eq!(0, price_response.colors[0]);
    }

    #[test]
    fn buy_pixel_test() {
      let mut deps = mock_dependencies(&coins(2, "token"));
      instantiate_contract(&mut deps);

      // it worked, let's query the state
      let res = query(deps.as_ref(), mock_env(), QueryMsg::GetColors {}).unwrap();
      let price_response: ColorsResponse = from_binary(&res).unwrap();
      assert_eq!(0, price_response.colors[0]);

      // Try buying a pixel
      let info = mock_info("buyer", &coins(1000, "token"));
      let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::BuyPixel { x: 0, y: 0, 
        color: 5, price: 100 * MILLION, for_sale: false, message: String::from("Hello, world!") }).unwrap();
      assert_eq!("try_buy", res.attributes[0].value);
      
      let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPixel { x: 0, y: 0 }).unwrap();
      let pixel_response: PixelResponse = from_binary(&res).unwrap();
      assert_eq!(5, pixel_response.color);
      assert_eq!(100 * MILLION, pixel_response.pixel_data.price);

      let info = mock_info("buyer", &coins(1000, "token"));
      let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::BuyPixel { x: 0, y: 0, 
        color: 5, price: 100 * MILLION, for_sale: false, message: String::from("Hello, world!") }).unwrap_err();
      assert_eq!("Generic error: Pixel data already exists", res.to_string());

      let res = query(deps.as_ref(), mock_env(), QueryMsg::GetColors {}).unwrap();
      let colors_response: ColorsResponse = from_binary(&res).unwrap();
      assert_eq!(5, colors_response.colors[0]);
      assert_eq!(25, colors_response.colors.len());
    }
}
