mod thumbnails;
mod categories;

use crate::datamodel::{Resources,ResourceProvider};
use std::vec::Vec;
use indicatif::ProgressBar;

pub fn generate(resources: &mut Resources)
{
    let outdated_resources : Vec<String> = 
     resources.resources.values()
    .filter_map(|x| 
        match &x.resource_provider{
            ResourceProvider::Generated(d) =>  if d.is_outdated(resources) {Some(x.id.clone()) } else {None} ,
            _ => None
        }
    ).collect();

    if outdated_resources.len() > 0
    {
        println!("Removing outdated generated content...");
        let bar = ProgressBar::new(outdated_resources.len().try_into().unwrap());
        bar.tick();
        for outdated_resource in outdated_resources
        {
            bar.inc(1);
            resources.remove_resource(&outdated_resource);
        }
        bar.finish();
    }

    thumbnails::generate(resources);
}