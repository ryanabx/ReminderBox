import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';
// import AccountCircle from '@mui/icons-material/AccountCircle';
// import Switch from '@mui/material/Switch';
// import FormControlLabel from '@mui/material/FormControlLabel';
// import FormGroup from '@mui/material/FormGroup';
import MenuItem from '@mui/material/MenuItem';
import Menu from '@mui/material/Menu';
import { useLocation, useNavigate } from 'react-router-dom';
import { MoreHoriz } from '@mui/icons-material';
import { FormControlLabel, FormGroup, Switch } from '@mui/material';

type HeaderProps = {
  showCompleted: boolean;
  setShowCompleted: (val: boolean) => void;
};

export default function Header({ showCompleted, setShowCompleted }: HeaderProps) {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const [anchorElRight, setAnchorElRight] = React.useState<null | HTMLElement>(null);

  const handleToggle = (event: React.ChangeEvent<HTMLInputElement, Element>) => {
    setShowCompleted(event.target.checked);
  };

  const navigate = useNavigate();

  const location = useLocation();

  const handleMenu = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleMenuRight = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorElRight(event.currentTarget);
  };

  const handleCloseRight = () => {
    setAnchorElRight(null);
  };

  const changePage = (page: string) => {
    handleClose();
    navigate(page);
  }

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="fixed">
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            aria-haspopup="true"
            sx={{ mr: 2 }}
            onClick={handleMenu}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" onClick={() => { changePage("/") }}
            component="div"
            sx={{ flexGrow: 1, cursor: "pointer" }}
          >
            ReminderBox
          </Typography>
          <div>
            <Menu
              id="menu-appbar"
              anchorEl={anchorEl}
              anchorOrigin={{
                vertical: 'bottom',
                horizontal: 'right',
              }}
              keepMounted
              transformOrigin={{
                vertical: 'top',
                horizontal: 'right',
              }}
              open={Boolean(anchorEl)}
              onClose={handleClose}
            >
              <MenuItem onClick={() => { changePage("/") }}>Reminders</MenuItem>
              <MenuItem onClick={() => changePage("/about/")}>About</MenuItem>
            </Menu>
          </div>
          {location.pathname === "/" && (
            <div>
              <IconButton
                size="large"
                aria-label="account of current user"
                aria-controls="menu-appbar"
                aria-haspopup="true"
                onClick={handleMenuRight}
                color="inherit"
              >
                <MoreHoriz />
              </IconButton>
              <Menu
                id="menu-appbar"
                anchorEl={anchorElRight}
                anchorOrigin={{
                  vertical: 'top',
                  horizontal: 'right',
                }}
                keepMounted
                transformOrigin={{
                  vertical: 'top',
                  horizontal: 'right',
                }}
                open={Boolean(anchorElRight)}
                onClose={handleCloseRight}
              >
                <MenuItem onClick={handleCloseRight}>
                  <FormGroup>
                    <FormControlLabel control={
                      <Switch
                        checked={showCompleted}
                        onChange={handleToggle}
                        edge="end"
                      />
                    } label="Show Completed" />
                  </FormGroup>
                </MenuItem>
              </Menu>
            </div>
          )
          }
        </Toolbar>
      </AppBar>
      <Toolbar />
    </Box>
  );
}