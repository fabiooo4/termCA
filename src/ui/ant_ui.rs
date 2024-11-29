use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    text::Span,
    widgets::{block::Position, Padding, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use rand::Rng;
use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        canvas::{Canvas, Points},
        Block, BorderType, Borders, Clear, Paragraph,
    },
};
use tui_widget_list::{ListBuilder, ListView};

use crate::{
    app::{App, EditTab, InputMode},
    simulations::{
        self,
        ant::{AntPresets, AntSettings, AntSim},
    },
};

use super::{centered_rect_length, render_help, settings_help, start_content, ListItemContainer};

pub fn ant_screen(frame: &mut Frame, app: &mut App) {
    if frame
        .area()
        .width
        .checked_mul(frame.area().height)
        .is_none()
    {
        let error_paragraph = Paragraph::new(
            "EEEEEEEEEEEEEEEEEEEEEE                                                                                                   tttt         hhhhhhh                                                         hhhhhhh                                                                                              tttt                                                                          iiii                                              iiii                                 tttt                                                                                                            lllllll lllllll 
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E                                                                                                ttt⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           ttt⣿⣿⣿t                                                                         i⣿⣿⣿⣿i                                            i⣿⣿⣿⣿i                             ttt⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E                                                                                                t⣿⣿⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           t⣿⣿⣿⣿⣿t                                                                          iiii                                              iiii                              t⣿⣿⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
EE⣿⣿⣿⣿⣿⣿EEEEEEEEE⣿⣿⣿⣿E                                                                                                t⣿⣿⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           t⣿⣿⣿⣿⣿t                                                                                                                                                              t⣿⣿⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E       EEEEEErrrrr   rrrrrrrrr   rrrrr   rrrrrrrrr      ooooooooooo   rrrrr   rrrrrrrrr                ttttttt⣿⣿⣿⣿⣿ttttttt    h⣿⣿⣿⣿h hhhhh           eeeeeeeeeeee             cccccccccccccccch⣿⣿⣿⣿h hhhhh         aaaaaaaaaaaaa  rrrrr   rrrrrrrrr   aaaaaaaaaaaaa      ccccccccccccccccttttttt⣿⣿⣿⣿⣿ttttttt        eeeeeeeeeeee    rrrrr   rrrrrrrrr            ssssssssss   iiiiiii zzzzzzzzzzzzzzzzz    eeeeeeeeeeee         iiiiiii     ssssssssss        ttttttt⣿⣿⣿⣿⣿ttttttt       ooooooooooo      ooooooooooo            ssssssssss      mmmmmmm    mmmmmmm     aaaaaaaaaaaaa    l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E             r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r  r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r   oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r   ⣿⣿⣿⣿⣿⣿      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    h⣿⣿⣿⣿hh⣿⣿⣿⣿⣿hhh      ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee         cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿hh⣿⣿⣿⣿⣿hhh      a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r  a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a   cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ct⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t      ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee  r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r         ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee       i⣿⣿⣿⣿⣿i   ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s       t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t     oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo        ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s   mm⣿⣿⣿⣿⣿⣿⣿m  m⣿⣿⣿⣿⣿⣿⣿mm   a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a   l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿EEEEEEEEEE   r⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r r⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿or⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r  ⣿⣿⣿⣿⣿⣿      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    h⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿hh   e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿ee      c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿hh    aaaaaaaaa⣿⣿⣿⣿⣿ar⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r aaaaaaaaa⣿⣿⣿⣿⣿a c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ct⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t     e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿eer⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r      ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿ee      i⣿⣿⣿⣿i ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o     ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s m⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿mm⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿m  aaaaaaaaa⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E   rr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿ro⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿orr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r ⣿⣿⣿⣿⣿⣿      tttttt⣿⣿⣿⣿⣿⣿⣿tttttt    h⣿⣿⣿⣿⣿⣿⣿hhh⣿⣿⣿⣿⣿⣿h e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿e     c⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿⣿⣿hhh⣿⣿⣿⣿⣿⣿h            a⣿⣿⣿⣿arr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r         a⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ctttttt⣿⣿⣿⣿⣿⣿⣿tttttt    e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿err⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r     s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿s i⣿⣿⣿⣿i zzzzzzzz⣿⣿⣿⣿⣿⣿z  e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿e      i⣿⣿⣿⣿i s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿s     tttttt⣿⣿⣿⣿⣿⣿⣿tttttt    o⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿sm⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿m           a⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E    r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿ro⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r                   t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿⣿h   h⣿⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e     c⣿⣿⣿⣿⣿⣿c     ccccccch⣿⣿⣿⣿⣿⣿h   h⣿⣿⣿⣿⣿⣿h    aaaaaaa⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r  aaaaaaa⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿c     ccccccc      t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r      s⣿⣿⣿⣿⣿s  ssssss  i⣿⣿⣿⣿i       z⣿⣿⣿⣿⣿⣿z   e⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e      i⣿⣿⣿⣿i  s⣿⣿⣿⣿⣿s  ssssss            t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o      s⣿⣿⣿⣿⣿s  ssssss m⣿⣿⣿⣿⣿mmm⣿⣿⣿⣿⣿⣿mmm⣿⣿⣿⣿⣿m    aaaaaaa⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿EEEEEEEEEE    r⣿⣿⣿⣿⣿r     rrrrrrr r⣿⣿⣿⣿⣿r     rrrrrrro⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r     rrrrrrr                   t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e      c⣿⣿⣿⣿⣿c             h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h  aa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r     rrrrrrraa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿c                   t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e  r⣿⣿⣿⣿⣿r     rrrrrrr        s⣿⣿⣿⣿⣿⣿s       i⣿⣿⣿⣿i      z⣿⣿⣿⣿⣿⣿z    e⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e       i⣿⣿⣿⣿i    s⣿⣿⣿⣿⣿⣿s                 t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o        s⣿⣿⣿⣿⣿⣿s      m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m  aa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E              r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r                               t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿eeeeeeeeeee       c⣿⣿⣿⣿⣿c             h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r           a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿c                   t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿eeeeeeeeeee   r⣿⣿⣿⣿⣿r                       s⣿⣿⣿⣿⣿⣿s    i⣿⣿⣿⣿i     z⣿⣿⣿⣿⣿⣿z     e⣿⣿⣿⣿⣿⣿eeeeeeeeeee        i⣿⣿⣿⣿i       s⣿⣿⣿⣿⣿⣿s              t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o           s⣿⣿⣿⣿⣿⣿s   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E       EEEEEE r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            t⣿⣿⣿⣿⣿t    tttttth⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿e                c⣿⣿⣿⣿⣿⣿c     ccccccch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿c     ccccccc      t⣿⣿⣿⣿⣿t    tttttte⣿⣿⣿⣿⣿⣿⣿e            r⣿⣿⣿⣿⣿r                 ssssss   s⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿i    z⣿⣿⣿⣿⣿⣿z      e⣿⣿⣿⣿⣿⣿⣿e                 i⣿⣿⣿⣿i ssssss   s⣿⣿⣿⣿⣿s            t⣿⣿⣿⣿⣿t    tttttto⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o     ssssss   s⣿⣿⣿⣿⣿s m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
EE⣿⣿⣿⣿⣿⣿EEEEEEEE⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿th⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿⣿e               c⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿c      t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿te⣿⣿⣿⣿⣿⣿⣿⣿e           r⣿⣿⣿⣿⣿r                 s⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿si⣿⣿⣿⣿⣿⣿i  z⣿⣿⣿⣿⣿⣿zzzzzzzze⣿⣿⣿⣿⣿⣿⣿⣿e               i⣿⣿⣿⣿⣿⣿is⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿s           t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿to⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿sm⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a l⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿th⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee        c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿c      tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee   r⣿⣿⣿⣿⣿r                 s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s i⣿⣿⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee       i⣿⣿⣿⣿⣿⣿is⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s            tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿to⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a l⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r             oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  r⣿⣿⣿⣿⣿r                                 tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tth⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e         cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿ar⣿⣿⣿⣿⣿r           a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿a cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿c        tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tt  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e   r⣿⣿⣿⣿⣿r                  s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss  i⣿⣿⣿⣿⣿⣿iz⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e       i⣿⣿⣿⣿⣿⣿i s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss               tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tt oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo       s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss  m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿al⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
EEEEEEEEEEEEEEEEEEEEEE rrrrrrr             rrrrrrr               ooooooooooo    rrrrrrr                                   ttttttttttt  hhhhhhh     hhhhhhh    eeeeeeeeeeeeee           cccccccccccccccchhhhhhh     hhhhhhh  aaaaaaaaaa  aaaarrrrrrr            aaaaaaaaaa  aaaa   cccccccccccccccc          ttttttttttt      eeeeeeeeeeeeee   rrrrrrr                   sssssssssss    iiiiiiiizzzzzzzzzzzzzzzzz    eeeeeeeeeeeeee       iiiiiiii  sssssssssss                   ttttttttttt     ooooooooooo      ooooooooooo          sssssssssss    mmmmmm   mmmmmm   mmmmmm  aaaaaaaaaa  aaaallllllllllllllll"
        ).alignment(Alignment::Center);
        frame.render_widget(
            error_paragraph,
            Rect {
                x: 0,
                y: 0,
                width: 592,
                height: 16,
            },
        );
        return;
    }

    // Initialize the ant simulation if it's not already
    let width = f64::from(frame.area().width - 2);
    let height = f64::from((frame.area().height - 2) * 2);

    if app.ant_sim.is_none() {
        app.start_ant_default();

        let sim = app.ant_sim.as_mut().unwrap();

        // Initialize the grid with the same size as the canvas
        sim.grid
            .resize(width as usize, height as usize, sim.states[0]);

        // Change default ruleset
        // ant_sim.rules = AntSim::parse_ant_ruleset("RRLLLRLLLLLLLLL");
        sim.rules = AntSim::parse_ant_ruleset("RL");

        // Set ant position randomly biased towards the center
        let mut rng = rand::thread_rng();

        for ant in &mut sim.ants {
            ant.x = rng.gen_range((width * 0.4) as usize..(width - width * 0.4) as usize) as usize;
            ant.y =
                rng.gen_range((height * 0.4) as usize..(height - height * 0.4) as usize) as usize;
        }

        // Set ant direction randomly
        for ant in &mut sim.ants {
            let direction = rng.gen_range(0..4);
            ant.direction = match direction {
                0 => simulations::Direction::Left,
                1 => simulations::Direction::Right,
                2 => simulations::Direction::Up,
                3 => simulations::Direction::Down,
                _ => simulations::Direction::Right,
            };
        }
    } else if app.ant_sim.as_ref().unwrap().generation == 0 {
        // If the ant simulation is already set, the grid still needs to be initialized with the
        // screen size
        let sim = app.ant_sim.as_mut().unwrap();

        // Initialize the grid with the same size as the canvas
        sim.grid
            .resize(width as usize, height as usize, sim.states[0]);

        // Reposition the ant inside the bounds if it is outside
        for ant in sim.ants.iter_mut() {
            if ant.x > width as usize {
                ant.x = width as usize / 2;
            }

            if ant.y > height as usize {
                ant.y = height as usize / 2;
            }
        }
    }

    // From here `app.ant_sim` is `Some`
    let sim = app.ant_sim.as_ref().unwrap();

    /////////////////////////////
    // Border content
    /////////////////////////////

    let top_title = Line::from(vec![" Langton's Ant ".yellow()]);

    let bottom_left_title = Line::from(vec![
        " Iteration: ".into(),
        sim.generation.to_string().yellow(),
        " ".into(),
    ]);

    let key_help = Line::from(vec![" '?' ".yellow(), "Help ".into()]);

    let bottom_right_title = Line::from(vec![
        " Speed: ".into(),
        if app.speed.as_millis() == 0 {
            format!("{}x ", app.speed_multiplier).yellow()
        } else {
            format!("{}ms ", app.speed.as_millis()).yellow()
        },
    ]);

    /////////////////////////////
    // Simulation canvas
    /////////////////////////////

    let ant_canvas = Canvas::default()
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .title_top(top_title.centered())
                .title_bottom(bottom_left_title.left_aligned())
                .title_bottom(bottom_right_title.right_aligned())
                .title_bottom(key_help.centered())
                .title_style(Style::default().bold()),
        )
        .marker(sim.marker)
        .paint(|ctx| {
            // Draw grid
            for (y, row) in sim.grid.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    match *cell {
                        // Skip drawing black cells
                        Color::Black => {}
                        _ => {
                            ctx.draw(&Points {
                                coords: &[(x as f64, y as f64)],
                                color: *cell,
                            });
                        }
                    }
                }
            }

            // Draw ant
            for ant in sim.ants.iter() {
                ctx.draw(&Points {
                    coords: &[(ant.x as f64, ant.y as f64)],
                    color: ant.color,
                });
            }
        })
        .x_bounds([0., f64::from((frame.area().width - 2) - 1)])
        .y_bounds([0., f64::from(((frame.area().height - 2) * 2) - 1)]);

    frame.render_widget(ant_canvas, frame.area());

    /////////////////////////////
    // Help screen
    /////////////////////////////

    let help_entries: Vec<(Line, Line)> = vec![
        (Line::from("?".yellow()), Line::from("Help")),
        (Line::from("Q / Esc".yellow()), Line::from("Quit")),
        (Line::from("Space".yellow()), Line::from("Start/Pause")),
        (Line::from("K / ↑".yellow()), Line::from("Speed Up")),
        (Line::from("J / ↓".yellow()), Line::from("Speed Down")),
        (Line::from("L / →".yellow()), Line::from("Next Generation")),
    ];

    if app.help_screen {
        render_help(frame, help_entries);
    }
}

pub fn edit_ant(frame: &mut Frame, app: &mut App, ant_idx: usize) {
    frame.render_widget(Clear, frame.area());
    if frame
        .area()
        .width
        .checked_mul(frame.area().height)
        .is_none()
    {
        let error_paragraph = Paragraph::new(
            "EEEEEEEEEEEEEEEEEEEEEE                                                                                                   tttt         hhhhhhh                                                         hhhhhhh                                                                                              tttt                                                                          iiii                                              iiii                                 tttt                                                                                                            lllllll lllllll 
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E                                                                                                ttt⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           ttt⣿⣿⣿t                                                                         i⣿⣿⣿⣿i                                            i⣿⣿⣿⣿i                             ttt⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E                                                                                                t⣿⣿⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           t⣿⣿⣿⣿⣿t                                                                          iiii                                              iiii                              t⣿⣿⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
EE⣿⣿⣿⣿⣿⣿EEEEEEEEE⣿⣿⣿⣿E                                                                                                t⣿⣿⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           t⣿⣿⣿⣿⣿t                                                                                                                                                              t⣿⣿⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E       EEEEEErrrrr   rrrrrrrrr   rrrrr   rrrrrrrrr      ooooooooooo   rrrrr   rrrrrrrrr                ttttttt⣿⣿⣿⣿⣿ttttttt    h⣿⣿⣿⣿h hhhhh           eeeeeeeeeeee             cccccccccccccccch⣿⣿⣿⣿h hhhhh         aaaaaaaaaaaaa  rrrrr   rrrrrrrrr   aaaaaaaaaaaaa      ccccccccccccccccttttttt⣿⣿⣿⣿⣿ttttttt        eeeeeeeeeeee    rrrrr   rrrrrrrrr            ssssssssss   iiiiiii zzzzzzzzzzzzzzzzz    eeeeeeeeeeee         iiiiiii     ssssssssss        ttttttt⣿⣿⣿⣿⣿ttttttt       ooooooooooo      ooooooooooo            ssssssssss      mmmmmmm    mmmmmmm     aaaaaaaaaaaaa    l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E             r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r  r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r   oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r   ⣿⣿⣿⣿⣿⣿      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    h⣿⣿⣿⣿hh⣿⣿⣿⣿⣿hhh      ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee         cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿hh⣿⣿⣿⣿⣿hhh      a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r  a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a   cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ct⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t      ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee  r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r         ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee       i⣿⣿⣿⣿⣿i   ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s       t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t     oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo        ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s   mm⣿⣿⣿⣿⣿⣿⣿m  m⣿⣿⣿⣿⣿⣿⣿mm   a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a   l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿EEEEEEEEEE   r⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r r⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿or⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r  ⣿⣿⣿⣿⣿⣿      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    h⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿hh   e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿ee      c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿hh    aaaaaaaaa⣿⣿⣿⣿⣿ar⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r aaaaaaaaa⣿⣿⣿⣿⣿a c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ct⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t     e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿eer⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r      ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿ee      i⣿⣿⣿⣿i ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o     ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s m⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿mm⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿m  aaaaaaaaa⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E   rr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿ro⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿orr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r ⣿⣿⣿⣿⣿⣿      tttttt⣿⣿⣿⣿⣿⣿⣿tttttt    h⣿⣿⣿⣿⣿⣿⣿hhh⣿⣿⣿⣿⣿⣿h e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿e     c⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿⣿⣿hhh⣿⣿⣿⣿⣿⣿h            a⣿⣿⣿⣿arr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r         a⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ctttttt⣿⣿⣿⣿⣿⣿⣿tttttt    e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿err⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r     s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿s i⣿⣿⣿⣿i zzzzzzzz⣿⣿⣿⣿⣿⣿z  e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿e      i⣿⣿⣿⣿i s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿s     tttttt⣿⣿⣿⣿⣿⣿⣿tttttt    o⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿sm⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿m           a⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E    r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿ro⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r                   t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿⣿h   h⣿⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e     c⣿⣿⣿⣿⣿⣿c     ccccccch⣿⣿⣿⣿⣿⣿h   h⣿⣿⣿⣿⣿⣿h    aaaaaaa⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r  aaaaaaa⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿c     ccccccc      t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r      s⣿⣿⣿⣿⣿s  ssssss  i⣿⣿⣿⣿i       z⣿⣿⣿⣿⣿⣿z   e⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e      i⣿⣿⣿⣿i  s⣿⣿⣿⣿⣿s  ssssss            t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o      s⣿⣿⣿⣿⣿s  ssssss m⣿⣿⣿⣿⣿mmm⣿⣿⣿⣿⣿⣿mmm⣿⣿⣿⣿⣿m    aaaaaaa⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿EEEEEEEEEE    r⣿⣿⣿⣿⣿r     rrrrrrr r⣿⣿⣿⣿⣿r     rrrrrrro⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r     rrrrrrr                   t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e      c⣿⣿⣿⣿⣿c             h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h  aa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r     rrrrrrraa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿c                   t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e  r⣿⣿⣿⣿⣿r     rrrrrrr        s⣿⣿⣿⣿⣿⣿s       i⣿⣿⣿⣿i      z⣿⣿⣿⣿⣿⣿z    e⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e       i⣿⣿⣿⣿i    s⣿⣿⣿⣿⣿⣿s                 t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o        s⣿⣿⣿⣿⣿⣿s      m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m  aa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E              r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r                               t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿eeeeeeeeeee       c⣿⣿⣿⣿⣿c             h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r           a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿c                   t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿eeeeeeeeeee   r⣿⣿⣿⣿⣿r                       s⣿⣿⣿⣿⣿⣿s    i⣿⣿⣿⣿i     z⣿⣿⣿⣿⣿⣿z     e⣿⣿⣿⣿⣿⣿eeeeeeeeeee        i⣿⣿⣿⣿i       s⣿⣿⣿⣿⣿⣿s              t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o           s⣿⣿⣿⣿⣿⣿s   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E       EEEEEE r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            t⣿⣿⣿⣿⣿t    tttttth⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿e                c⣿⣿⣿⣿⣿⣿c     ccccccch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿c     ccccccc      t⣿⣿⣿⣿⣿t    tttttte⣿⣿⣿⣿⣿⣿⣿e            r⣿⣿⣿⣿⣿r                 ssssss   s⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿i    z⣿⣿⣿⣿⣿⣿z      e⣿⣿⣿⣿⣿⣿⣿e                 i⣿⣿⣿⣿i ssssss   s⣿⣿⣿⣿⣿s            t⣿⣿⣿⣿⣿t    tttttto⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o     ssssss   s⣿⣿⣿⣿⣿s m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
EE⣿⣿⣿⣿⣿⣿EEEEEEEE⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿th⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿⣿e               c⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿c      t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿te⣿⣿⣿⣿⣿⣿⣿⣿e           r⣿⣿⣿⣿⣿r                 s⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿si⣿⣿⣿⣿⣿⣿i  z⣿⣿⣿⣿⣿⣿zzzzzzzze⣿⣿⣿⣿⣿⣿⣿⣿e               i⣿⣿⣿⣿⣿⣿is⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿s           t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿to⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿sm⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a l⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿th⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee        c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿c      tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee   r⣿⣿⣿⣿⣿r                 s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s i⣿⣿⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee       i⣿⣿⣿⣿⣿⣿is⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s            tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿to⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a l⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r             oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  r⣿⣿⣿⣿⣿r                                 tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tth⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e         cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿ar⣿⣿⣿⣿⣿r           a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿a cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿c        tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tt  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e   r⣿⣿⣿⣿⣿r                  s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss  i⣿⣿⣿⣿⣿⣿iz⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e       i⣿⣿⣿⣿⣿⣿i s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss               tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tt oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo       s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss  m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿al⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
EEEEEEEEEEEEEEEEEEEEEE rrrrrrr             rrrrrrr               ooooooooooo    rrrrrrr                                   ttttttttttt  hhhhhhh     hhhhhhh    eeeeeeeeeeeeee           cccccccccccccccchhhhhhh     hhhhhhh  aaaaaaaaaa  aaaarrrrrrr            aaaaaaaaaa  aaaa   cccccccccccccccc          ttttttttttt      eeeeeeeeeeeeee   rrrrrrr                   sssssssssss    iiiiiiiizzzzzzzzzzzzzzzzz    eeeeeeeeeeeeee       iiiiiiii  sssssssssss                   ttttttttttt     ooooooooooo      ooooooooooo          sssssssssss    mmmmmm   mmmmmm   mmmmmm  aaaaaaaaaa  aaaallllllllllllllll"
        ).alignment(Alignment::Center);
        frame.render_widget(
            error_paragraph,
            Rect {
                x: 0,
                y: 0,
                width: 592,
                height: 16,
            },
        );
        return;
    }

    let width = f64::from(frame.area().width - 2);
    let height = f64::from((frame.area().height - 2) * 2);

    // If the ant simulation is already set, the grid still needs to be initialized with the
    // screen size
    let sim = app.ant_sim.as_mut().unwrap();

    // Initialize the grid with the same size as the canvas
    sim.grid
        .resize(width as usize, height as usize, sim.states[0]);

    // Wrap ant position
    for ant in sim.ants.iter_mut() {
        if ant.x > width as usize {
            ant.x = width as usize / 2;
        }

        if ant.y > height as usize {
            ant.y = height as usize / 2;
        }
    }

    let sim = app.ant_sim.as_ref().unwrap();

    /////////////////////////////
    // Border content
    /////////////////////////////

    let top_title = Line::from(vec![format!(" Editing Ant {} position ", ant_idx).yellow()]);

    let right_style = Style::default().bold().yellow();
    let left_style = Style::default().bold().red();
    let up_style = Style::default().bold().blue();
    let down_style = Style::default().bold().green();

    let bottom_left_title = Line::from(vec![
        " Direction: ".into(),
        Span::from(sim.ants[ant_idx].direction.to_string()).style(
            match sim.ants[ant_idx].direction {
                crate::simulations::Direction::Right => right_style,
                crate::simulations::Direction::Left => left_style,
                crate::simulations::Direction::Up => up_style,
                crate::simulations::Direction::Down => down_style,
            },
        ),
        " ".into(),
    ]);

    let help_label = Line::from(vec![" '?' ".yellow(), "Help ".into()]);

    let bottom_right_title = Line::from(vec![
        " Position: ".into(),
        format!("(x: {}, y: {}) ", sim.ants[ant_idx].x, sim.ants[ant_idx].y).into(),
    ]);

    /////////////////////////////
    // Simulation canvas
    /////////////////////////////

    let ant_canvas = Canvas::default()
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .title_top(top_title.centered())
                .title_bottom(bottom_left_title.left_aligned())
                .title_bottom(bottom_right_title.right_aligned())
                .title_bottom(help_label.centered())
                .title_style(Style::default().bold()),
        )
        .marker(sim.marker)
        .paint(|ctx| {
            // Draw ants
            for (i, ant) in sim.ants.iter().enumerate() {
                ctx.draw(&Points {
                    coords: &[(ant.x as f64, ant.y as f64)],
                    color: match sim.ants[i].direction {
                        crate::simulations::Direction::Right => Color::Yellow,
                        crate::simulations::Direction::Left => Color::Red,
                        crate::simulations::Direction::Up => Color::Blue,
                        crate::simulations::Direction::Down => Color::Green,
                    },
                });
            }
        })
        .x_bounds([0., f64::from((frame.area().width - 2) - 1)])
        .y_bounds([0., f64::from(((frame.area().height - 2) * 2) - 1)]);

    frame.render_widget(ant_canvas, frame.area());

    /////////////////////////////
    // Help screen
    /////////////////////////////

    let help_entries: Vec<(Line, Line)> = vec![
        (Line::from("?".yellow()), Line::from("Help")),
        (
            Line::from("Q / Esc / Enter".yellow()),
            Line::from("Save position"),
        ),
        (Line::from("K / ↑".yellow()), Line::from("Move up")),
        (Line::from("J / ↓".yellow()), Line::from("Move down")),
        (Line::from("L / →".yellow()), Line::from("Move right")),
        (Line::from("H / ←".yellow()), Line::from("Move left")),
        (Line::from("r".yellow()), Line::from("Rotate left")),
        (Line::from("R".yellow()), Line::from("Rotate right")),
    ];

    if app.help_screen {
        render_help(frame, help_entries);
    }
}

pub fn edit(frame: &mut Frame, app: &mut App) {
    let sim = app.ant_sim.as_mut().unwrap();

    /////////////////////////////
    // Centered popup
    /////////////////////////////
    let settings = [
        AntSettings::Ruleset.to_string(),
        AntSettings::Ants.to_string(),
        AntSettings::Start.to_string(),
    ];
    let longest_setting: u16 = settings
        .iter()
        .map(|k| k.to_string().len() as u16)
        .max()
        .unwrap_or(1);

    let edit_area = centered_rect_length(
        longest_setting + 38,
        AntSettings::COUNT as u16 * 2 + 4,
        frame.area(),
    );

    let edit_block = Block::default()
        .title(" Editing Lanton's Ant ".bold())
        .title_alignment(Alignment::Center)
        .title_position(Position::Bottom);

    frame.render_widget(Clear, edit_area);

    /////////////////////////////
    // Layouts
    /////////////////////////////

    let [top, bottom] =
        Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(edit_area);

    let [left, right] =
        Layout::horizontal([Constraint::Length(longest_setting + 4), Constraint::Min(0)])
            .areas(bottom);

    frame.render_widget(edit_block, top);

    /////////////////////////////
    // Block styles
    /////////////////////////////

    let selected_block = ratatui::widgets::Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().yellow().bold())
        .padding(Padding::horizontal(1))
        .style(Style::default().not_dim());

    let disabled_block = ratatui::widgets::Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .padding(Padding::horizontal(1))
        .style(Style::default().dim());

    /////////////////////////////
    // Settings list
    /////////////////////////////

    let block = match app.selected_edit_tab.as_ref().unwrap() {
        EditTab::Setting => selected_block.clone(),
        _ => disabled_block.clone(),
    };

    frame.render_stateful_widget(
        AntSettingsList::build_list().block(block.clone()),
        left,
        &mut sim.settings_state,
    );

    /////////////////////////////
    // Content
    /////////////////////////////

    let block = match app.selected_edit_tab.as_ref().unwrap() {
        EditTab::Content => selected_block.clone(),
        _ => disabled_block.clone(),
    };

    frame.render_widget(block, right);

    let [right_content] = Layout::vertical([Constraint::Fill(1)])
        .vertical_margin(1)
        .horizontal_margin(2)
        .areas(right);

    match AntSettings::from_index(sim.settings_state.selected.unwrap_or(0)) {
        AntSettings::Presets => {
            presets_content(
                sim,
                app.selected_edit_tab.as_ref().unwrap(),
                frame,
                right,
                right_content,
            );
        }
        AntSettings::Ruleset => {
            ruleset_content(right_content, sim, app.help_screen, frame);
        }
        AntSettings::Ants => {
            ants_list_content(
                app.selected_edit_tab.as_ref().unwrap(),
                frame,
                sim,
                right_content,
            );
        }
        AntSettings::Start => {
            start_content(frame, right_content);
        }
    }

    /////////////////////////////
    // Help screen
    /////////////////////////////

    let help_entries: Vec<(Line, Line)> = match app.selected_edit_tab.as_ref().unwrap() {
        EditTab::Setting => settings_help(),
        EditTab::Content => {
            match AntSettings::from_index(sim.settings_state.selected.unwrap_or(0)) {
                AntSettings::Presets => presets_help(),
                AntSettings::Ruleset => ruleset_help(),
                AntSettings::Ants => ants_help(),
                AntSettings::Start => vec![],
            }
        }
    };

    if app.help_screen {
        render_help(frame, help_entries);
    }
}

fn presets_help<'a>() -> Vec<(Line<'a>, Line<'a>)> {
    vec![
        (Line::from("?".yellow()), Line::from("Help")),
        (
            Line::from("Q / Esc / H / ←".yellow()),
            Line::from("Back to settings"),
        ),
        (Line::from("Enter".yellow()), Line::from("Select item")),
        (Line::from("K / ↑".yellow()), Line::from("Previous item")),
        (Line::from("J / ↓".yellow()), Line::from("Next item")),
    ]
}

fn ruleset_help<'a>() -> Vec<(Line<'a>, Line<'a>)> {
    vec![
        (Line::from("?".yellow()), Line::from("Help")),
        (
            Line::from("Q / Esc / Enter / H".yellow()),
            Line::from("Stop typing"),
        ),
    ]
}

fn ants_help<'a>() -> Vec<(Line<'a>, Line<'a>)> {
    vec![
        (Line::from("?".yellow()), Line::from("Help")),
        (
            Line::from("Q / Esc / H / ←".yellow()),
            Line::from("Back to settings"),
        ),
        (Line::from("Enter".yellow()), Line::from("Select item")),
        (Line::from("K / ↑".yellow()), Line::from("Previous item")),
        (Line::from("J / ↓".yellow()), Line::from("Next item")),
        (
            Line::from("Backspace".yellow()),
            Line::from("Delete selected ant"),
        ),
    ]
}

fn presets_content(
    sim: &mut AntSim,
    selected_tab: &EditTab,
    frame: &mut Frame<'_>,
    buf: Rect,
    inner_buf: Rect,
) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

    sim.preset_scroll_state = sim.preset_scroll_state.content_length(AntPresets::COUNT);
    sim.preset_scroll_state = sim
        .preset_scroll_state
        .position(sim.preset_state.selected.unwrap());

    if *selected_tab == EditTab::Content {
        frame.render_stateful_widget(
            scrollbar,
            buf.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut sim.preset_scroll_state,
        );
    }

    frame.render_stateful_widget(AntPresets::build_list(), inner_buf, &mut sim.preset_state);
}

fn ruleset_content(buf: Rect, sim: &mut AntSim, help_screen: bool, frame: &mut Frame<'_>) {
    let info = Paragraph::new(vec![
        Line::from("The list of possible rules is:"),
        Line::from("R: Turn right"),
        Line::from("L: Turn left"),
        Line::from("F: Continue forward"),
        Line::from("B: Opposite direction"),
    ])
    .dim()
    .wrap(Wrap { trim: true });

    let [info_chunk, input_chunk] =
        Layout::vertical([Constraint::Min(1), Constraint::Max(3)]).areas(buf);

    let scroll = sim
        .rules_input
        .visual_scroll(buf.width.saturating_sub(3) as usize);

    let input = Paragraph::new(sim.rules_input.value())
        .scroll((0, scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(match sim.rules_input_mode {
                    InputMode::Normal => Style::default().dim(),
                    InputMode::Editing => Style::default().bold().not_dim(),
                }),
        );

    match sim.rules_input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            if !help_screen {
                {
                    frame.set_cursor_position((
                        // Put cursor past the end of the input text
                        buf.x
                            + ((sim.rules_input.visual_cursor()).saturating_sub(scroll)) as u16
                            + 1,
                        // Move one line down, from the border to the input line
                        buf.y + 6,
                    ))
                }
            }
        }
    }

    frame.render_widget(info, info_chunk);
    frame.render_widget(input, input_chunk);
}

fn ants_list_content(selected_tab: &EditTab, frame: &mut Frame<'_>, sim: &mut AntSim, buf: Rect) {
    frame.render_stateful_widget(
        sim.build_ants_list(selected_tab),
        buf,
        &mut sim.ants_list_state,
    );
}

impl AntSim {
    pub fn build_ants_list<'a>(
        &self,
        selected_edit_tab: &EditTab,
    ) -> ListView<'a, ListItemContainer<'a, Paragraph<'a>>> {
        let is_content_selected = match selected_edit_tab {
            EditTab::Setting => false,
            EditTab::Content => true,
        };

        let ants = self.ants.clone();
        let grid_width = self.grid.width();
        let grid_height = self.grid.height();

        let right_style = Style::default().bold().yellow();
        let left_style = Style::default().bold().red();
        let up_style = Style::default().bold().blue();
        let down_style = Style::default().bold().green();

        let builder = ListBuilder::new(move |context| {
            if context.index == 0 {
                // The first item is the add button
                let label = Paragraph::new("+ Add ant").centered().block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );
                let mut add_button = ListItemContainer::new(label, Padding::bottom(1));
                if context.is_selected && is_content_selected {
                    add_button = add_button.yellow().bold();
                };

                (add_button, 4)
            } else {
                // The others are the ants

                let ant = ants.get(context.index.saturating_sub(1)).unwrap();

                let line = Paragraph::new(vec![
                    Line::from(vec![Span::from(format!("Ant {}:", context.index))]),
                    Line::from(vec![
                        Span::from(format!(
                            "({},{}) ",
                            match ant.x {
                                usize::MAX => "Center".to_string(),
                                _ => {
                                    if ant.x == grid_width / 2 {
                                        "Center".to_string()
                                    } else {
                                        ant.x.to_string()
                                    }
                                }
                            },
                            match ant.y {
                                usize::MAX => "Center".to_string(),
                                _ => {
                                    if ant.y == grid_height / 2 {
                                        "Center".to_string()
                                    } else {
                                        ant.y.to_string()
                                    }
                                }
                            }
                        )),
                        Span::from(ant.direction.to_string()).style(match ant.direction {
                            crate::simulations::Direction::Right => right_style,
                            crate::simulations::Direction::Left => left_style,
                            crate::simulations::Direction::Up => up_style,
                            crate::simulations::Direction::Down => down_style,
                        }),
                    ]),
                ])
                .centered();
                let mut item = ListItemContainer::new(line, Padding::ZERO);
                if context.is_selected && is_content_selected {
                    item = item.yellow().bold();
                };

                (item, 3)
            }
        });

        ListView::new(builder, self.ants.len() + 1)
    }
}

pub struct AntSettingsList;
impl AntSettingsList {
    pub fn build_list<'a>() -> ListView<'a, ListItemContainer<'a, Line<'a>>> {
        let builder = ListBuilder::new(move |context| {
            let config = AntSettings::from_index(context.index);
            let line = Line::from(format!("{config}")).centered();
            let mut item = ListItemContainer::new(line, Padding::ZERO);

            if context.is_selected {
                item = item.yellow().bold();
            };

            (item, 2)
        });

        ListView::new(builder, AntSettings::COUNT)
    }
}

impl AntPresets {
    pub fn build_list<'a>() -> ListView<'a, ListItemContainer<'a, Line<'a>>> {
        let builder = ListBuilder::new(move |context| {
            let config = AntPresets::from_index(context.index);
            let line = Line::from(format!("{config}")).centered();
            let mut item = ListItemContainer::new(line, Padding::ZERO);

            if context.is_selected {
                item = item.yellow().bold();
            };

            (item, 2)
        });

        ListView::new(builder, AntPresets::COUNT)
    }
}
